use lasso::Spur;

use crate::{bump::Node, token::Symbol, Spanned};

pub type Module = Node<Spanned<Block>>;

pub enum Expr {
    Import(Spanned<Import>),
    Let(Spanned<Let>),
    If(Spanned<If>),
    While(Spanned<While>),
    For(Spanned<For>),
    Loop(Spanned<Loop>),
    Continue,
    Break(Option<Node<Spanned<Expr>>>),
    Return(Option<Node<Spanned<Expr>>>),
    Paren(Node<Spanned<Expr>>),
    // fn name(x: ty) -> ty
    //   ...
    FuncDecl(Spanned<NamedFunc>),
    // fn(x: ty) -> ty =>
    AnonFunc(Spanned<AnonFunc>),
    // fn ty:method(x: ty) -> ty
    //   ...
    Method(Spanned<Method>),
    StructDef(Spanned<StructDef>),
    StructInit(Spanned<StructInit>),
    ListInit(Spanned<ListInit>),

    // var
    Ident(Spur),
    // 12 | 0xc | 0b1100
    Int(i64),
    // 1.0 | 1.0e10 | 1.0e-10
    Float(f64),
    // "string"
    String(Spur),
    // true | false
    Bool(bool),
    // [var, var, var]
    List(Vec<Spanned<Expr>>),
    // var + var
    Binary(Binary),
    // -var
    Unary(Unary),
    // var()
    Call(Call),
    // var.x
    Access(Access),
    // var[x]
    Index(Index),
}

pub struct ItemPath {
    pub items: Vec<Spanned<PathPart>>,
}

pub enum PathPart {
    // `foo` and `bar` in `self::foo::bar`
    Name(Spur),
    // `self` in `self::foo`
    Self_,
    // `super` in `super::foo`
    Super,
    // `root` in `root::foo`
    Root,
}

pub struct Import {
    // `std:time` in `import std:time`
    pub path: ItemPath,
    // `t` in `import std:time as t`
    pub alias: Option<Spur>,
}

pub enum TypeSig {
    Unit,
    Int,
    Float,
    String,
    Bool,
    List(Box<TypeSig>),
    Tuple(Vec<TypeSig>),
    Func(Vec<TypeSig>, Box<TypeSig>),
    Struct(Vec<(Spur, TypeSig)>),
    Enum(Vec<(Spur, TypeSig)>),
}

pub enum TypeName {
    Unit,
    Int,
    Float,
    String,
    Bool,
    Tuple(Vec<TypeName>),
    List(Box<TypeName>),
    Func(Vec<TypeName>, Option<Box<TypeName>>),
    // Struct or enum
    Named(ItemPath),
}

pub struct StructDef {
    pub name: Spur,
    pub fields: Vec<(Spanned<Spur>, Spanned<TypeName>)>,
}

pub struct EnumDef {
    pub name: Spur,
    pub variants: Vec<(Spur, EnumVariant)>,
}

pub struct TupleInit {
    pub items: Vec<Node<Spanned<Expr>>>,
}

pub struct StructInit {
    pub name: Option<Spanned<Spur>>,
    pub fields: Vec<(Spanned<Spur>, Node<Spanned<Expr>>)>,
}

pub enum EnumVariant {
    Unit,
    Tuple(Vec<Spanned<TypeName>>),
    Struct(Vec<(Spanned<Spur>, Node<Spanned<TypeName>>)>),
}

pub struct ListInit {
    pub items: Vec<Node<Spanned<Expr>>>,
}

pub struct While {
    pub cond: Node<Spanned<Expr>>,
    pub body: Node<Spanned<Block>>,
}

pub struct If {
    pub cond: Node<Spanned<Expr>>,
    pub body: Node<Spanned<Block>>,
    pub alt: Option<Node<Spanned<Expr>>>,
}

pub struct Let {
    pub pat: Node<Spanned<Expr>>,
    pub init: Option<Node<Spanned<Expr>>>,
}

pub struct Loop {
    pub body: Node<Spanned<Block>>,
}

pub struct For {
    // Ident or destructuring expr
    pub item: Node<Spanned<Expr>>,
    pub iter: Node<Spanned<Expr>>,
    pub body: Node<Spanned<Block>>,
}

pub struct Block {
    pub stmts: Vec<Node<Spanned<Expr>>>,
}

pub struct NamedFunc {
    pub name: Spanned<Spur>,
    pub args: Vec<(Spanned<Spur>, Spanned<TypeName>)>,
    pub body: Node<Spanned<Block>>,
}

pub struct Method {
    pub ty: Spanned<TypeName>,
    pub name: Spanned<Spur>,
    pub args: Vec<(Spanned<Spur>, Spanned<TypeName>)>,
    pub body: Node<Spanned<Block>>,
    // Whether the method is static (has no self param)
    // Static methods are called with Type:method() instead of value.method().
    pub is_static: bool,
}

pub struct AnonFunc {
    pub args: Vec<(Spanned<Spur>, Spanned<TypeName>)>,
    pub body: Node<Spanned<Block>>,
}

pub struct Binary {
    pub op: Spanned<Symbol>,
    pub lhs: Node<Spanned<Expr>>,
    pub rhs: Node<Spanned<Expr>>,
}

pub struct Unary {
    pub op: Symbol,
    pub expr: Node<Spanned<Expr>>,
}

pub struct Call {
    pub func: Node<Spanned<Expr>>,
    pub args: Vec<Spanned<Expr>>,
}

pub struct Access {
    pub expr: Node<Spanned<Expr>>,
    pub field: Spanned<Spur>,
}

pub struct Index {
    pub expr: Node<Spanned<Expr>>,
    pub index: Node<Spanned<Expr>>,
}
