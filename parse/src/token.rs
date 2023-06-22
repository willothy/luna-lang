use std::{fmt::Display, marker::PhantomData, path::Path};

use ariadne::Source;
use chumsky::{
    extra::Full,
    input::{BoxedStream, Offset, StrInput, ValueInput},
    prelude::{Input, Rich},
    primitive::{any, choice, empty, end, just, todo},
    recovery::skip_then_retry_until,
    span::{SimpleSpan, Span as _},
    text, ConfigIterParser, IterParser, Parser,
};
use internment::ArenaIntern;

use crate::{
    arena::Id,
    span::{FileCache, Span, Spanned},
};

#[derive(Debug, Clone, Copy)]
pub enum Token<'a> {
    Int(i64),
    Nat(u64),
    Float(f64),
    String(ArenaIntern<'a, String>),
    Char(char),
    Bool(bool),
    Ident(ArenaIntern<'a, String>),
    Op(Op),
    Symbol(Symbol),
    Keyword(Keyword),
    /// Opening delimiters, ( [ < {
    Open(Delim),
    /// Closing delimiters, ) ] > }
    Close(Delim),
    /// =
    /// or
    /// += -= *= /= %=
    /// ?=
    Assign(Option<Op>),
    Error(char),
}

impl<'a> Token<'a> {
    pub fn spanned(self, span: Span) -> Spanned<Self> {
        (self, span)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Op {
    //
    // Comparison
    //
    /// ==
    Eq,
    /// !=
    Neq,
    /// <
    Lt,
    /// >
    Gt,
    /// <=
    Leq,
    /// >=
    Geq,
    //
    // Arithmetic
    //
    /// +
    Add,
    /// -
    Sub,
    /// *
    Mul,
    /// /
    Div,
    /// %
    Mod,
    //
    // Logical
    //
    /// and
    And,
    /// or
    Or,
    /// not
    Not,
}

#[derive(Debug, Clone, Copy)]
pub enum Delim {
    Paren,
    Bracket,
    Brace,
    Angle,
}

#[derive(Debug, Clone, Copy)]
pub enum Symbol {
    /// .
    Dot,
    /// ..
    /// Used as a concat operator, or to represent a range
    Concat,
    /// ..=
    Unpack,
    /// :
    Colon,
    /// ::
    DoubleColon,
    /// ->
    RArrow,
    /// <-
    LArrow,
    /// =>
    FatArrow,
    /// ?
    /// Used to mark/check optional values
    Optional,
    /// |
    Pipe,
    /// \
    Backslash,
    /// ,
    Comma,
}

#[derive(Debug, Clone, Copy)]
pub enum Keyword {
    Fn,
    Type,
    Import,
    Class,
    /// self
    SelfParam,
    /// Self
    SelfType,
    Let,
    Match,
    With,
    As,
    If,
    Then,
    Else,
    For,
    In,
    While,
    Loop,
    Break,
    Continue,
    Return,
}

impl<'a> Display for Token<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Token::Nat(i) => write!(f, "{}", i),
            Token::Int(i) => write!(f, "{}", i),
            Token::Float(v) => write!(f, "{}", v),
            Token::String(s) => write!(f, "\"{}\"", s),
            Token::Char(c) => write!(f, "'{}'", c),
            Token::Bool(b) => write!(f, "{}", if b { "true" } else { "false" }),
            Token::Ident(s) => write!(f, "{}", s),
            Token::Op(op) => write!(f, "{}", op),
            Token::Symbol(sym) => write!(f, "{}", sym),
            Token::Keyword(kw) => write!(f, "{}", kw),
            Token::Open(d) => match d {
                Delim::Paren => write!(f, "("),
                Delim::Bracket => write!(f, "["),
                Delim::Brace => write!(f, "{{"),
                Delim::Angle => write!(f, "<"),
            },
            Token::Close(d) => match d {
                Delim::Paren => write!(f, ")"),
                Delim::Bracket => write!(f, "]"),
                Delim::Brace => write!(f, "}}"),
                Delim::Angle => write!(f, ">"),
            },
            Token::Assign(complex) => match complex {
                Some(op) => write!(f, "{}=", op),
                None => write!(f, "="),
            },
            Token::Error(c) => write!(f, "{}", c),
        }
    }
}

impl Display for Op {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Op::Eq => write!(f, "=="),
            Op::Neq => write!(f, "!="),
            Op::Lt => write!(f, "<"),
            Op::Gt => write!(f, ">"),
            Op::Leq => write!(f, "<="),
            Op::Geq => write!(f, ">="),
            Op::Add => write!(f, "+"),
            Op::Sub => write!(f, "-"),
            Op::Mul => write!(f, "*"),
            Op::Div => write!(f, "/"),
            Op::Mod => write!(f, "%"),
            Op::And => write!(f, "and"),
            Op::Or => write!(f, "or"),
            Op::Not => write!(f, "not"),
        }
    }
}

impl Display for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Symbol::Dot => write!(f, "."),
            Symbol::Concat => write!(f, ".."),
            Symbol::Unpack => write!(f, "..="),
            Symbol::Colon => write!(f, ":"),
            Symbol::DoubleColon => write!(f, "::"),
            Symbol::RArrow => write!(f, "->"),
            Symbol::LArrow => write!(f, "<-"),
            Symbol::FatArrow => write!(f, "=>"),
            Symbol::Optional => write!(f, "?"),
            Symbol::Pipe => write!(f, "|"),
            Symbol::Backslash => write!(f, "\\"),
            Symbol::Comma => write!(f, ","),
        }
    }
}

impl Display for Keyword {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Keyword::Fn => write!(f, "fn"),
            Keyword::Type => write!(f, "type"),
            Keyword::Import => write!(f, "import"),
            Keyword::Class => write!(f, "class"),
            Keyword::SelfParam => write!(f, "self"),
            Keyword::SelfType => write!(f, "Self"),
            Keyword::Let => write!(f, "let"),
            Keyword::Match => write!(f, "match"),
            Keyword::With => write!(f, "with"),
            Keyword::As => write!(f, "as"),
            Keyword::If => write!(f, "if"),
            Keyword::Then => write!(f, "then"),
            Keyword::Else => write!(f, "else"),
            Keyword::For => write!(f, "for"),
            Keyword::In => write!(f, "in"),
            Keyword::While => write!(f, "while"),
            Keyword::Loop => write!(f, "loop"),
            Keyword::Break => write!(f, "break"),
            Keyword::Continue => write!(f, "continue"),
            Keyword::Return => write!(f, "return"),
        }
    }
}

pub struct LexerState {
    interner: internment::Arena<String>,
}

impl Default for LexerState {
    fn default() -> Self {
        LexerState {
            interner: internment::Arena::new(),
        }
    }
}

pub type Output<'a> = Vec<Spanned<Token<'a>>>;
pub type Extra<'a> = Full<Rich<'a, char, Span>, LexerState, ()>;

pub fn lexer<'a, I>() -> impl Parser<'a, I, Output<'a>, Extra<'a>>
where
    I: StrInput<'a, char, Offset = usize, Span = Span>,
{
    let float = text::int(10)
        .then_ignore(just('.'))
        .then(text::int(10))
        .map(|(v, d): (&str, &str)| Token::Float(format!("{v}.{d}").parse::<f64>().unwrap()));

    let token = choice((
        float,
        // todo
        //
    ))
    .or(any().map(Token::Error))
    .map_with_span(|tok, span| (tok, span))
    .padded();

    token.repeated().collect().padded().then_ignore(end())
}

#[test]
fn t() {
    let sources = FileCache::new();
    let workspace = Path::new(env!("CARGO_MANIFEST_DIR")).join("test.luna");
    let path = workspace;
    let source = sources.resolve(&path).unwrap();
    let mut state = LexerState {
        interner: internment::Arena::new(),
    };
    let code = sources.get(source).chars().collect::<String>();
    let res = lexer().parse_with_state(code.as_str().with_context(source), &mut state);

    if res.has_errors() {
        res.errors().for_each(|e| {
            println!("{:?}", e);
        });
    }
    if let Some(output) = res.output() {
        println!("{:#?}", output);
    }
    assert!(false)
}
