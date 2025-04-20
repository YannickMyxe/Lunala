use crate::tokens::TokenType;

pub struct Expression {
    expression_type: ExpType,
    
}

#[derive(Debug)]
pub enum ExpType {
    Literal(Literal),
    Unary {
        operator: TokenType,
        expression: Box<ExpType>,
    },
    Binary {
        operator: TokenType,
        left: Box<ExpType>,
        right: Box<ExpType>,
    },
    Grouping {
        expression: Box<ExpType>,
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

impl ExpType {
}
