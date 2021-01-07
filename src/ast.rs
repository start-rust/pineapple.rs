#[derive(PartialEq, Clone, Debug)]
pub struct Ident(pub String);

#[derive(PartialEq, Clone, Debug)]
pub enum Literal {
    String(String),
}

#[derive(PartialEq, Clone, Debug)]
pub enum Expr {
    Ident(Ident),
    Index(Box<Expr>, Box<Expr>),
    Literal(Literal),
    Call {
        func: Box<Expr>,
        args: Vec<Expr>,
    },
}

#[derive(PartialEq, Clone, Debug)]
pub enum Stmt {
    Blank,
    Let(Ident, Expr),
    Return(Expr),
    Expr(Expr),
}

pub type BlockStmt = Vec<Stmt>;

pub type Program = BlockStmt;

#[derive(PartialEq, PartialOrd, Debug, Clone)]
pub enum Precedence {
    Lowest,
    Call,        // myFunction(x)
}
