use std::fmt;
use crate::ast::Expr;

#[derive(Debug, Clone)]
pub enum Value {
    Number(f64),
    String(String),
    Boolean(bool),
    Array(Vec<Value>),
    Function {
        params: Vec<String>,
        body: Vec<Expr>,
    },
    Transformer {
        params: Vec<String>,
        body: Vec<Expr>,
    },
    Nil,
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Number(n) => write!(f, "{}", n),
            Value::String(s) => write!(f, "{}", s),
            Value::Boolean(b) => write!(f, "{}", b),
            Value::Array(elements) => {
                write!(f, "[")?;
                for (i, element) in elements.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", element)?;
                }
                write!(f, "]")
            },
            Value::Function { .. } => write!(f, "<function>"),
            Value::Transformer { .. } => write!(f, "<transformer>"),
            Value::Nil => write!(f, "nil"),
        }
    }
}
