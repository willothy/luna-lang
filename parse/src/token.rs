use std::fmt::Display;

use internment::Intern;

use crate::span::{Span, Spanned};

#[derive(Debug, Clone)]
pub enum Token {
    Int(i64),
    Nat(u64),
    Float(f64),
    String(Intern<String>),
    Char(char),
    Bool(bool),
    Ident(Intern<String>),
    Wildcard,
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
    Error(String),
    Indent(usize),
    Newline,
}

impl Token {
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
    /// ..
    /// Used as a concat operator, or to represent a range
    Concat,
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
    /// !
    Bang,
}

#[derive(Debug, Clone, Copy)]
pub enum Keyword {
    Fn,
    Type,
    Import,
    Struct,
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
    Enum,
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Newline => write!(f, "Newline"),
            Token::Indent(i) => write!(f, "Indent {}", i),
            Token::Nat(i) => write!(f, "{}", i),
            Token::Int(i) => write!(f, "{}", i),
            Token::Float(v) => write!(f, "{}", v),
            Token::String(s) => write!(f, "\"{}\"", s),
            Token::Char(c) => write!(f, "'{}'", c),
            Token::Bool(b) => write!(f, "{}", if *b { "true" } else { "false" }),
            Token::Ident(s) => write!(f, "{}", s),
            Token::Op(op) => write!(f, "{}", op),
            Token::Symbol(sym) => write!(f, "{}", sym),
            Token::Keyword(kw) => write!(f, "{}", kw),
            Token::Wildcard => write!(f, "_"),
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
            Op::Concat => write!(f, ".."),
        }
    }
}

impl Display for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Symbol::Dot => write!(f, "."),
            Symbol::Colon => write!(f, ":"),
            Symbol::DoubleColon => write!(f, "::"),
            Symbol::RArrow => write!(f, "->"),
            Symbol::LArrow => write!(f, "<-"),
            Symbol::FatArrow => write!(f, "=>"),
            Symbol::Optional => write!(f, "?"),
            Symbol::Pipe => write!(f, "|"),
            Symbol::Backslash => write!(f, "\\"),
            Symbol::Comma => write!(f, ","),
            Symbol::Bang => write!(f, "!"),
        }
    }
}

impl Display for Keyword {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Keyword::Fn => write!(f, "fn"),
            Keyword::Type => write!(f, "type"),
            Keyword::Import => write!(f, "import"),
            Keyword::Struct => write!(f, "struct"),
            Keyword::Enum => write!(f, "enum"),
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
