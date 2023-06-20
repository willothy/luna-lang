use chumsky::input::{BoxedStream, SpannedInput, Stream};
use chumsky::primitive::{choice, just, todo};
use chumsky::recursive::recursive;
use chumsky::select;
use chumsky::span::SimpleSpan;
use chumsky::{extra::Full, prelude::Rich, Parser as Parse};
use lasso::Rodeo;

use crate::ast::{Block, Expr, If, Module, While};
use crate::token::*;
use crate::{bump::BumpMap, lexer::Tokens, token::Token, Spanned};

#[macro_export]
macro_rules! kw {
    (@$id:ident) => {
        chumsky::primitive::just(Token::Keyword(Keyword::$id))
    };
    ($id:ident) => {
        Token::Keyword(Keyword::$id)
    };
}

#[macro_export]
macro_rules! sym {
    (@$id:ident) => {
        chumsky::primitive::just(crane_lex::Token::Symbol(Symbol::$id))
    };
    ($id:ident) => {
        Token::Symbol(Symbol::$id)
    };
}

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

pub type Input<'a> = SpannedInput<Token, SimpleSpan, BoxedStream<'a, Spanned<Token>>>;
pub type State = ParserState;
pub type Extra<'a> = Full<Rich<'a, Token>, State, ()>;

pub trait Parser<'a, Output = Spanned<Block>> = chumsky::Parser<'a, Input<'a>, Output, Extra<'a>>;

// pub fn expr<'a>() -> impl Parser<'a, Spanned<Expr>> {
//     recursive(|expr| {
//         let r#if = recursive(|r#if| {
//             kw!(@If)
//                 .ignore_then(
//                     expr.clone()
//                         .map_with_state(|v, _, s: &mut ParserState| s.nodes.insert(v)),
//                 )
//                 .then(
//                     expr.clone()
//                         .map_with_state(|v, _, s: &mut ParserState| s.nodes.insert(v)),
//                 )
//                 .then(kw!(@Else).ignore_then(expr.clone()).or_not())
//                 .map_with_state(|(cond, body, alt), _, s: &mut ParserState| {
//                     let node = Expr::If(If { cond, body, alt });
//                     s.nodes.insert(node)
//                 })
//         });
//
//         let r#while = kw!(@While)
//             .ignore_then(expr.clone())
//             .then(expr.clone())
//             .map_with_state(|(cond, body), _, s: &mut ParserState| {
//                 let node = Expr::While(While { cond, body });
//                 s.nodes.insert(node)
//             });
//
//         let atom = choice((
//             select! {
//                 Token::Int(i) => Expr::Int(i),
//                 Token::Float(f) => Expr::Float(f),
//                 Token::Str(s) => Expr::String(s),
//                 Token::Bool(b) => Expr::Bool(b),
//                 Token::Ident(i) => Expr::Ident(i),
//             },
//             just(Token::Open(Delim::Paren))
//                 .ignore_then(expr.clone())
//                 .then_ignore(just(Token::Close(Delim::Paren)))
//                 .map(|expr| Expr::Paren(expr)),
//             r#if,
//             r#while,
//         ));
//
//         atom.map_with_span(|span, expr| (expr, span))
//     })
// }
//
// pub fn block<'a>() -> impl Parser<'a> {
//     todo()
// }
//
// pub fn module<'a>() -> impl Parser<'a, Spanned<Module>> {
//     todo()
// }
