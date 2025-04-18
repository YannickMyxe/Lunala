use crate::tokens::TokenType;

pub struct Tree {
    root: Expression,
}

#[derive(Debug)]
pub enum Expression {
    Literal(Literal),
    Unary {
        operator: TokenType,
        expression: Box<Expression>,
    },
    Binary {
        operator: TokenType,
        left: Box<Expression>,
        right: Box<Expression>,
    },
    Grouping {
        expression: Box<Expression>,
    }
}

#[derive(Debug)]
pub enum Literal {
    Number(i32),
    String(String),
}

pub enum Number {
    Int(Integer),
    Float(Float),
}

pub enum Integer {
    I16, I32, I64, I128,
}

pub enum Float {
    F16, F32, F64, F128,
}

impl Tree {
    pub fn new(root: Expression) -> Self {
        Self { root }
    }
}

impl Expression {
}
