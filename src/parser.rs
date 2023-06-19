use chumsky::{extra::Full, prelude::Rich, Parser as Parse};
use lasso::Rodeo;

use crate::ast::Module;
use crate::{bump::BumpMap, lexer::Tokens, token::Token, Spanned};

pub struct ParserState {
    interner: Rodeo,
    nodes: BumpMap,
}

impl ParserState {
    pub fn new() -> Self {
        Self {
            interner: Rodeo::default(),
            nodes: BumpMap::new(),
        }
    }
}

pub type State = ParserState;
pub type Extra<'a> = Full<Rich<'a, Spanned<Token>>, State, ()>;

pub trait Parser<'a> {}
impl<'a, P: Parse<'a, Tokens<'a>, Module, Extra<'a>>> Parser<'a> for P {}

pub fn parse_block<'a>() -> impl Parser<'a> {
    chumsky::primitive::todo()
}
