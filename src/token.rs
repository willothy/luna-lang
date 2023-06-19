use std::fmt::Display;

use lasso::Spur;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Ident(Spur),
    Int(u64),
    Float(f64),
    Str(Spur),
    Open(Delim),
    Close(Delim),
    Symbol(Symbol),
    Keyword(Keyword),
    Bool(bool),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Symbol {
    Colon,
    DoubleColon,
    Dot,
    Comma,
    Arrow,
    FatArrow,
    Plus,
    Minus,
    Times,
    Divide,
    Modulo,
    Assign,
    Optional,
    Bang,
    Concat,
    And,
    Or,
    BitAnd,
    BitOr,
    Xor,
    LShift,
    RShift,
    Eq,
    Neq,
    Lt,
    Gt,
    Leq,
    Geq,
    PlusEq,
    MinusEq,
    TimesEq,
    DivideEq,
    ModuloEq,
    ConcatEq,
    BitAndEq,
    BitOrEq,
    XorEq,
    LShiftEq,
    RShiftEq,
    InitAssign,
}

impl Display for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Symbol::Colon => write!(f, ":"),
            Symbol::DoubleColon => write!(f, "::"),
            Symbol::Dot => write!(f, "."),
            Symbol::Comma => write!(f, ","),
            Symbol::Arrow => write!(f, "->"),
            Symbol::FatArrow => write!(f, "=>"),
            Symbol::Plus => write!(f, "+"),
            Symbol::Minus => write!(f, "-"),
            Symbol::Times => write!(f, "*"),
            Symbol::Divide => write!(f, "/"),
            Symbol::Modulo => write!(f, "%"),
            Symbol::Assign => write!(f, "="),
            Symbol::Optional => write!(f, "?"),
            Symbol::Bang => write!(f, "!"),
            Symbol::Concat => write!(f, ".."),
            Symbol::And => write!(f, "&&"),
            Symbol::Or => write!(f, "||"),
            Symbol::BitAnd => write!(f, "&"),
            Symbol::BitOr => write!(f, "|"),
            Symbol::Xor => write!(f, "^"),
            Symbol::LShift => write!(f, "<<"),
            Symbol::RShift => write!(f, ">>"),
            Symbol::Eq => write!(f, "=="),
            Symbol::Neq => write!(f, "!="),
            Symbol::Lt => write!(f, "<"),
            Symbol::Gt => write!(f, ">"),
            Symbol::Leq => write!(f, "<="),
            Symbol::Geq => write!(f, ">="),
            Symbol::PlusEq => write!(f, "+="),
            Symbol::MinusEq => write!(f, "-="),
            Symbol::TimesEq => write!(f, "*="),
            Symbol::DivideEq => write!(f, "/="),
            Symbol::ModuloEq => write!(f, "%="),
            Symbol::ConcatEq => write!(f, "..="),
            Symbol::BitAndEq => write!(f, "&="),
            Symbol::BitOrEq => write!(f, "|="),
            Symbol::XorEq => write!(f, "^="),
            Symbol::LShiftEq => write!(f, "<<="),
            Symbol::RShiftEq => write!(f, ">>="),
            Symbol::InitAssign => write!(f, "?="),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Delim {
    Paren,
    Bracket,
    Brace,
    Block,
}

impl Display for Delim {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Delim::Paren => write!(f, "Paren"),
            Delim::Bracket => write!(f, "Bracket"),
            Delim::Brace => write!(f, "Brace"),
            Delim::Block => write!(f, "Block"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Keyword {
    Fn,
    Pub,
    Import,
    Struct,
    Trait,
    Impl,
    For,
    In,
    If,
    Else,
    While,
    Loop,
    Break,
    Continue,
    Return,
    Global,
    Let,
}

impl Display for Keyword {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Keyword::Fn => write!(f, "fn"),
            Keyword::Pub => write!(f, "pub"),
            Keyword::Import => write!(f, "import"),
            Keyword::Struct => write!(f, "struct"),
            Keyword::Trait => write!(f, "trait"),
            Keyword::Impl => write!(f, "impl"),
            Keyword::For => write!(f, "for"),
            Keyword::In => write!(f, "in"),
            Keyword::If => write!(f, "if"),
            Keyword::Else => write!(f, "else"),
            Keyword::While => write!(f, "while"),
            Keyword::Loop => write!(f, "loop"),
            Keyword::Break => write!(f, "break"),
            Keyword::Continue => write!(f, "continue"),
            Keyword::Return => write!(f, "return"),
            Keyword::Global => write!(f, "global"),
            Keyword::Let => write!(f, "let"),
        }
    }
}
