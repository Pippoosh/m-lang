use crate::token::Token;

// AST Node types
#[derive(Debug, Clone)]
pub enum Expr {
    Number(f64),
    String(String),
    Boolean(bool),
    Array(Vec<Expr>),
    Variable(String),
    Binary {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    Unary {
        operator: Token,
        right: Box<Expr>,
    },
    Assign {
        name: String,
        value: Box<Expr>,
    },
    Call {
        callee: String,
        arguments: Vec<Expr>,
    },
    Function {
        name: String,
        params: Vec<String>,
        body: Vec<Expr>,
    },
    Return {
        value: Option<Box<Expr>>,
    },
    Block(Vec<Expr>),
    If {
        condition: Box<Expr>,
        then_branch: Box<Expr>,
        else_branch: Option<Box<Expr>>,
    },
    For {
        variable: String,
        iterable: Box<Expr>,
        body: Box<Expr>,
    },
    Index {
        object: Box<Expr>,
        index: Box<Expr>,
    },
    While {
        condition: Box<Expr>,
        body: Box<Expr>,
    },
    Transformer {
        name: String,
        params: Vec<String>,
        body: Vec<Expr>,
    },
    Apply {
        object: Box<Expr>,
        transformer: String,
        arguments: Vec<Expr>,
    },
    Use {
        path: String,
    },
}
