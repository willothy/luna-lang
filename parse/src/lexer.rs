use chumsky::{
    extra::Full,
    input::StrInput,
    prelude::Rich,
    primitive::{any, choice, end, just, none_of},
    text::{self, newline, unicode::ident},
    IterParser, Parser,
};
use internment::Intern;

use crate::{
    span::{Span, Spanned},
    token::{Delim, Keyword, Op, Symbol, Token},
};

pub struct LexerState {}

impl LexerState {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for LexerState {
    fn default() -> Self {
        Self {}
    }
}

pub type Output<'a> = Vec<Spanned<Token>>;
pub type Extra<'a> = Full<Rich<'a, char, Span>, LexerState, ()>;

pub fn lexer<'a, I>() -> impl Parser<'a, I, Output<'a>, Extra<'a>>
where
    I: StrInput<'a, char, Offset = usize, Span = Span>,
{
    let float = text::int(10)
        .then_ignore(just('.'))
        .then(text::int(10))
        .map(|(v, d)| format!("{v}.{d}"))
        .map(|v| {
            v.parse::<f64>()
                .map(Token::Float)
                .unwrap_or(Token::Error(v))
        })
        .labelled("float");

    let int = text::int(10)
        .then_ignore(just('i').or_not())
        .map(|v: &str| {
            v.parse::<i64>()
                .map(Token::Int)
                .unwrap_or(Token::Error(v.to_owned()))
        })
        .labelled("int");

    let concat = just("..").to(Op::Concat);

    let arithmetic = choice((
        just("+").to(Op::Add),
        just("-").to(Op::Sub),
        just("*").to(Op::Mul),
        just("/").to(Op::Div),
        just("%").to(Op::Mod),
    ));

    let comparison = choice((
        just("!=").to(Op::Neq),
        just("==").to(Op::Eq),
        just("<=").to(Op::Leq),
        just(">=").to(Op::Geq),
        just(">").to(Op::Gt),
        just("<").to(Op::Lt),
    ));

    let logical = choice((
        just("not").to(Op::Not),
        just("and").to(Op::And),
        just("or").to(Op::Or),
    ));

    let op = choice((
        concat,     // ..
        arithmetic, // + - * / %
        comparison, // > < >= <= == !=
        logical,    // and or not
    ))
    .map(Token::Op);

    // Complex assignments can include arithmetic and the concat operator only
    let assign = arithmetic
        .or(concat)
        .or_not()
        .then_ignore(just('='))
        .map(Token::Assign)
        .labelled("assignment");

    let word = ident()
        .map(|v| match v {
            "fn" => Token::Keyword(Keyword::Fn),
            "type" => Token::Keyword(Keyword::Type),
            "import" => Token::Keyword(Keyword::Import),
            "struct" => Token::Keyword(Keyword::Struct),
            "enum" => Token::Keyword(Keyword::Enum),
            "self" => Token::Keyword(Keyword::SelfParam),
            "Self" => Token::Keyword(Keyword::SelfType),
            "let" => Token::Keyword(Keyword::Let),
            "match" => Token::Keyword(Keyword::Match),
            "with" => Token::Keyword(Keyword::With),
            "as" => Token::Keyword(Keyword::As),
            "if" => Token::Keyword(Keyword::If),
            "then" => Token::Keyword(Keyword::Then),
            "else" => Token::Keyword(Keyword::Else),
            "for" => Token::Keyword(Keyword::For),
            "in" => Token::Keyword(Keyword::In),
            "while" => Token::Keyword(Keyword::While),
            "loop" => Token::Keyword(Keyword::Loop),
            "break" => Token::Keyword(Keyword::Break),
            "continue" => Token::Keyword(Keyword::Continue),
            "return" => Token::Keyword(Keyword::Return),
            "_" => Token::Wildcard,
            _ => Token::Ident(Intern::new(v.to_owned())),
        })
        .labelled("word");

    let sym = choice((
        just("::").to(Symbol::DoubleColon),
        just(":").to(Symbol::Colon),
        just("->").to(Symbol::RArrow),
        just("<-").to(Symbol::LArrow),
        just("=>").to(Symbol::FatArrow),
        just("?").to(Symbol::Optional),
        just("|").to(Symbol::Pipe),
        just("\\").to(Symbol::Backslash),
        just(",").to(Symbol::Comma),
    ))
    .map(Token::Symbol)
    .labelled("symbol");

    let sym2 = choice((
        // lower priority symbols that should be checked after ops
        just(".").to(Symbol::Dot),
        just("!").to(Symbol::Bang),
    ))
    .map(Token::Symbol)
    .labelled("symbol");

    let delim = choice((
        just('(').to(Delim::Paren).map(Token::Open),
        just(')').to(Delim::Paren).map(Token::Close),
        just('[').to(Delim::Bracket).map(Token::Open),
        just(']').to(Delim::Bracket).map(Token::Close),
        just('{').to(Delim::Brace).map(Token::Open),
        just('}').to(Delim::Brace).map(Token::Close),
    ))
    .labelled("delim");

    let escape = just('\\').ignore_then(
        just('\\')
            .or(just('/'))
            .or(just('"'))
            .or(just('b').to('\x08'))
            .or(just('r').to('\r'))
            .or(just('n').to('\n'))
            .or(just('t').to('\t')),
    );

    let char = just('\'')
        .ignore_then(none_of("\\\'").or(escape))
        .then_ignore(just('\''))
        .map(Token::Char)
        .labelled("character");

    let string = just('"')
        .ignore_then(none_of("\\\"").or(escape).repeated().collect::<String>())
        .then_ignore(just('"'))
        .map(Intern::new)
        .map(Token::String)
        .labelled("string");

    let comment = just('#')
        .then_ignore(newline().not().repeated())
        .padded()
        .ignored()
        .repeated();

    let indent = just(' ').repeated().count().map(Token::Indent);

    let newline = just(newline().repeated().at_least(1)).to(Token::Newline);

    let token = choice((
        word, // keyword or ident
        string, char, // strings
        float, int, // numeric
        sym, assign, op, sym2, delim, // symbols
        indent, // indentation
               // newline, // newline
    ))
    .or(any().map(|c: char| c.to_string()).map(Token::Error))
    .map_with_span(|tok, span| (tok, span))
    .padded();

    token
        .padded_by(comment)
        .repeated()
        .collect()
        .padded()
        .then_ignore(end())
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use chumsky::{prelude::Input, Parser};

    use crate::{
        lexer::{lexer, LexerState},
        span::FileCache,
    };

    #[test]
    fn t() {
        let sources = FileCache::new();
        let workspace = Path::new(env!("CARGO_MANIFEST_DIR")).join("test.luna");
        let path = workspace;
        let source = sources.resolve(&path).unwrap();
        let mut state = LexerState::new();
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
}
