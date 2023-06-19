use chumsky::primitive::none_of;
use chumsky::recursive::recursive;
use chumsky::span::SimpleSpan;
use chumsky::ParseResult;
use chumsky::{
    extra::Full,
    input::BoxedStream,
    prelude::Rich,
    primitive::{choice, just},
    text::{self, ascii::keyword},
    IterParser, Parser,
};
use lasso::{Rodeo, Spur};

use crate::indent::semantic_indentation;
use crate::token::Delim;
use crate::{
    token::{Keyword, Symbol, Token},
    Spanned,
};

pub type Tokens<'a> = BoxedStream<'a, Spanned<Token>>;
pub type Extra<'a> = Full<Rich<'a, char>, Rodeo<Spur>, ()>;
pub trait Tokenizer<'a, O> = Parser<'a, &'a str, O, Extra<'a>> + Clone;
pub trait Atom<'a> = Tokenizer<'a, Token>;

fn ident<'a>() -> impl Atom<'a> {
    text::unicode::ident()
        .map_with_state(|str, _, s: &mut Rodeo| Token::Ident(s.get_or_intern(str)))
}

fn kw<'a>() -> impl Atom<'a> {
    choice((
        keyword("if").to(Keyword::If),
        keyword("else").to(Keyword::Else),
        keyword("while").to(Keyword::While),
        keyword("for").to(Keyword::For),
        keyword("loop").to(Keyword::Loop),
        keyword("in").to(Keyword::In),
        keyword("break").to(Keyword::Break),
        keyword("continue").to(Keyword::Continue),
        keyword("return").to(Keyword::Return),
        keyword("global").to(Keyword::Global),
        keyword("let").to(Keyword::Let),
        keyword("import").to(Keyword::Import),
        keyword("pub").to(Keyword::Pub),
        keyword("struct").to(Keyword::Struct),
        keyword("trait").to(Keyword::Trait),
        keyword("impl").to(Keyword::Impl),
        keyword("fn").to(Keyword::Fn),
    ))
    .map(Token::Keyword)
}

pub fn sym<'a>() -> impl Atom<'a> {
    choice((
        just("::").to(Symbol::DoubleColon),
        just(":").to(Symbol::Colon),
        just(",").to(Symbol::Comma),
        just(".").to(Symbol::Dot),
        just("->").to(Symbol::Arrow),
        just("=>").to(Symbol::FatArrow),
        choice((
            just("+"),
            just("-"),
            just("*"),
            just("/"),
            just("%"),
            just(">>"),
            just("<<"),
            just("&"),
            just("|"),
            just("^"),
            just(">"),
            just("<"),
            just("!"),
            just("="),
            just(".."),
            just("?"),
        ))
        .then(just('=').ignored().or_not())
        .map(|(op, eq)| {
            macro_rules! select {
                ($a:ident, $b:ident) => {
                    eq.map_or($crate::token::Symbol::$b, |_| $crate::token::Symbol::$a)
                };
            }
            match op {
                "+" => select!(PlusEq, Plus),
                "-" => select!(MinusEq, Minus),
                "*" => select!(TimesEq, Times),
                "/" => select!(DivideEq, Divide),
                "%" => select!(ModuloEq, Modulo),
                ">>" => select!(RShiftEq, RShift),
                "<<" => select!(LShiftEq, LShift),
                "&" => select!(BitAndEq, BitAnd),
                "|" => select!(BitOrEq, BitOr),
                "^" => select!(XorEq, Xor),
                ">" => select!(Geq, Gt),
                "<" => select!(Leq, Lt),
                "!" => select!(Neq, Bang),
                "=" => select!(Eq, Assign),
                ".." => select!(ConcatEq, Concat),
                "?" => select!(InitAssign, Optional),
                _ => unreachable!(),
            }
        }),
    ))
    .map(Token::Symbol)
}

pub fn int<'a>() -> impl Atom<'a> {
    text::int(10).from_str().unwrapped().map(Token::Int)
}

pub fn int_hex<'a>() -> impl Atom<'a> {
    text::int(16).from_str().unwrapped().map(Token::Int)
}

pub fn int_bin<'a>() -> impl Atom<'a> {
    text::int(2).from_str().unwrapped().map(Token::Int)
}

pub fn int_oct<'a>() -> impl Atom<'a> {
    text::int(8).from_str().unwrapped().map(Token::Int)
}

pub fn float<'a>() -> impl Atom<'a> {
    text::int(10)
        .then_ignore(just('.'))
        .then(text::int(10))
        .map(|(n, dec)| format!("{}.{}", n, dec))
        .from_str()
        .unwrapped()
        .map(Token::Float)
}

pub fn float_scientific<'a>() -> impl Atom<'a> {
    text::int(10)
        .then_ignore(just('.'))
        .then(text::int(10))
        .then_ignore(choice((just('e'), just('E'))))
        .then(choice((just('+'), just('-'))).or_not())
        .then(text::int(10))
        .map(|(((num, decimal), sign), exp)| {
            format!("{}.{}e{}{}", num, decimal, sign.unwrap_or('+'), exp)
        })
        .from_str()
        .unwrapped()
        .map(Token::Float)
}

pub fn string<'a>() -> impl Atom<'a> {
    none_of("\"")
        .repeated()
        .collect::<String>()
        .delimited_by(just('"'), just('"'))
        .map_with_state(|str, _, s: &mut Rodeo| Token::Str(s.get_or_intern(str)))
}

pub fn bool<'a>() -> impl Atom<'a> {
    choice((keyword("true"), keyword("false"))).map_with_state(|str, _, _| match str {
        "true" => Token::Bool(true),
        _ => Token::Bool(false),
    })
}

pub fn token<'a>() -> impl Atom<'a> {
    kw().or(sym())
        .or(string())
        .or(bool())
        .or(ident())
        .or(float_scientific())
        .or(float())
        .or(int())
        .or(int_hex())
        .or(int_bin())
        .or(int_oct())
}

pub enum TokenTree {
    Token(Token),
    Tree(Delim, Vec<Spanned<TokenTree>>),
}

pub trait Flatten {
    fn flatten(self) -> Vec<Spanned<Token>>;
}

impl Flatten for Spanned<TokenTree> {
    fn flatten(self) -> Vec<Spanned<Token>> {
        match self.0 {
            TokenTree::Token(t) => vec![(t, self.1)],
            TokenTree::Tree(d, tts) => {
                let mut tokens =
                    vec![(Token::Open(d), SimpleSpan::new(self.1.start, self.1.start))];
                let mut last = self.1.end;
                for tt in tts {
                    last = tt.1.end;
                    tokens.extend(tt.flatten());
                }
                tokens.push((Token::Close(d), SimpleSpan::new(last, last)));
                tokens
            }
        }
    }
}

impl Flatten for Vec<Spanned<TokenTree>> {
    fn flatten(self) -> Vec<Spanned<Token>> {
        let mut tokens = Vec::new();
        for tt in self {
            tokens.extend(tt.flatten());
        }
        tokens
    }
}

pub fn lexer<'a>() -> impl Tokenizer<'a, Vec<Spanned<Token>>> {
    let tt = recursive(|tt| {
        let token_tree = tt
            .padded()
            .repeated()
            .collect::<Vec<_>>()
            .delimited_by(just('('), just(')'))
            .map(|tts| TokenTree::Tree(Delim::Paren, tts));

        token()
            .map(TokenTree::Token)
            .or(token_tree)
            .map_with_span(|tt, span| (tt, span))
    });

    semantic_indentation(tt, |tts, span| (TokenTree::Tree(Delim::Block, tts), span))
        .map(|tt| tt.flatten())
}

pub struct Lexer {
    rodeo: Rodeo<Spur>,
}

impl Lexer {
    pub fn new() -> Self {
        Self {
            rodeo: Rodeo::default(),
        }
    }

    pub fn lex<'a>(&mut self, chunk: &'a str) -> ParseResult<Vec<Spanned<Token>>, Rich<'a, char>> {
        lexer().parse_with_state(chunk, &mut self.rodeo)
    }
}

pub fn print_tokens(tokens: &[Spanned<Token>], rodeo: &Rodeo) {
    for (token, span) in tokens {
        match token {
            Token::Ident(key) => println!("Ident: {} at {}", rodeo.resolve(&key), span),
            Token::Int(v) => println!("Int: {} at {}", v, span),
            Token::Float(v) => println!("Float: {} at {}", v, span),
            Token::Str(v) => println!("Str: {} at {}", rodeo.resolve(&v), span),
            Token::Open(v) => println!("Open: {} at {}", v, span),
            Token::Close(v) => println!("Close: {} at {}", v, span),
            Token::Symbol(v) => println!("Symbol: {} at {}", v, span),
            Token::Keyword(v) => println!("Keyword: {} at {}", v, span),
            Token::Bool(v) => println!("Bool: {} at {}", v, span),
        }
    }
}
