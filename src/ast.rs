#![allow(clippy::upper_case_acronyms, dead_code)]

use pest::Span;

#[derive(Debug, Clone)]
pub struct Ident<'a> {
    pub name: String,
    pub span: Span<'a>,
}

impl<'a> PartialEq for Ident<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl<'a> Eq for Ident<'a> {}
#[derive(Debug, Clone)]
pub enum Item<'a> {
    Node(Node<'a>),
    Def(Def<'a>),
}

pub type Items<'a> = Vec<Item<'a>>;

#[derive(Debug, Clone)]
pub struct Param<'a> {
    pub ident: Ident<'a>,
}

pub type Params<'a> = Vec<Param<'a>>;

#[derive(Debug, Clone)]
pub struct Def<'a> {
    pub ident: Ident<'a>,
    pub params: Params<'a>,
    pub items: Items<'a>,
}

impl<'a> Def<'a> {
    pub fn is_function(&self) -> bool {
        !self.params.is_empty()
    }
}

#[derive(Debug, Clone)]
pub enum Node<'a> {
    Term(Term<'a>),
    BinExpr(BinExpr<'a>),
    UnExpr(UnExpr<'a>),
    Call(CallExpr<'a>),
    Insert(InsertExpr<'a>),
    Get(GetExpr<'a>),
}

#[derive(Debug, Clone)]
pub struct BinExpr<'a> {
    pub left: Box<Node<'a>>,
    pub right: Box<Node<'a>>,
    pub op: BinOp,
    pub span: Span<'a>,
}

impl<'a> BinExpr<'a> {
    pub fn new(left: Node<'a>, right: Node<'a>, op: BinOp, span: Span<'a>) -> Self {
        BinExpr {
            left: left.into(),
            right: right.into(),
            op,
            span,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinOp {
    Or,
    And,
    Is,
    Isnt,
    Less,
    LessOrEqual,
    Greater,
    GreaterOrEqual,
    Add,
    Sub,
    Mul,
    Div,
    Rem,
}

#[derive(Debug, Clone)]
pub struct UnExpr<'a> {
    pub inner: Box<Node<'a>>,
    pub op: UnOp,
}

impl<'a> UnExpr<'a> {
    pub fn new(inner: Node<'a>, op: UnOp) -> Self {
        UnExpr {
            inner: inner.into(),
            op,
        }
    }
}

#[derive(Debug, Clone)]
pub enum UnOp {
    Not,
    Neg,
}

#[derive(Debug, Clone)]
pub struct CallExpr<'a> {
    pub expr: Box<Node<'a>>,
    pub args: Vec<Node<'a>>,
    pub chained: Option<String>,
    pub span: Span<'a>,
}

#[derive(Debug, Clone)]
pub struct Insertion<'a> {
    pub key: Access<'a>,
    pub val: Node<'a>,
}

#[derive(Debug, Clone)]
pub struct InsertExpr<'a> {
    pub inner: Box<Node<'a>>,
    pub insertions: Vec<Insertion<'a>>,
}

#[derive(Debug, Clone)]
pub enum Access<'a> {
    Index(Term<'a>),
    Field(Ident<'a>),
}

#[derive(Debug, Clone)]
pub struct GetExpr<'a> {
    pub inner: Box<Node<'a>>,
    pub access: Access<'a>,
}

#[derive(Debug, Clone)]
pub enum Term<'a> {
    Expr(Items<'a>),
    Int(i64),
    Real(f64),
    Ident(Ident<'a>),
    Bool(bool),
    String(String),
    Closure(Box<Closure<'a>>),
    Nil,
}

#[derive(Debug, Clone)]
pub struct Closure<'a> {
    pub span: Span<'a>,
    pub params: Params<'a>,
    pub body: Items<'a>,
}
