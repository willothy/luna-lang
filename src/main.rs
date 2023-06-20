#![feature(trait_alias)]

/// Luna example:
///
/// ```luna
/// import std:time
///
/// pub struct Person ::
///     name: string
///     age: int
///     bday: DateTime
///
/// pub fn Person:new(name: string) -> Person
///     Person!
///         name
///         bday: time.now()
///
/// pub fn Person:age_up(self)
///     self.name += 1
///
/// pub trait Identify ::
///     fn identify(self) -> string
///
/// impl Identify for Person ::
///     fn identify(self) -> string
///         self.name
///
/// global people: [Person] = []
///
/// let jim = Person:new("Jim")
///
/// people.push(jim)
///
/// people.iter().for_each(fn(p: Person) -> void :: p.age_up())
///
/// for person in people
///     person.identify()
/// ```
pub mod ast;
pub mod bump;
pub mod indent;
pub mod lexer;
pub mod parser;
pub mod token;

pub type Spanned<T> = (T, SimpleSpan);

use chumsky::span::SimpleSpan;
use lasso::Rodeo;
use lexer::{Lexer, PrintTokens};

fn main() {
    let code = "\
import std:time

pub struct Person ::
    name: string
    age: int
    bday: DateTime

pub fn Person:new(name: string) -> Person
    Person!
        name
        bday: time.now()

pub fn Person:age_up(self)
    self.name += 1

pub trait Identify ::
    fn identify(self) -> string

impl Identify for Person ::
    fn identify(self) -> string
        self.name

global people: [Person] = []

let jim = Person:new(\"Jim\")

people.push(jim)

people.iter().for_each(fn(p: Person) -> void :: p.age_up())

for person in people
    person.identify()
";

    let mut rodeo = Rodeo::new();
    let tokens = Lexer::new(&mut rodeo).lex(code).unwrap();
    tokens.print(&rodeo);
}
