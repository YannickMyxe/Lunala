use crate::errors::{ErrorTypes, LunalaErrors};
use crate::tokens::Token;

#[derive(Debug)]
pub struct Expression {
    expression_type: ExpType,
}

#[derive(Debug, Clone)]
pub enum ExpType {
    Literal(Literal),
    Unary {
        operator: Token,
        expression: Box<ExpType>,
    },
    Binary {
        operator: Token,
        left: Box<ExpType>,
        right: Box<ExpType>,
    },
    Grouping {
        expression: Box<ExpType>,
    }
}

#[derive(Debug, Clone)]
pub enum Literal {
    Number(i32),
    Bool(bool),
    String(String),
}

pub enum _Number {
    Int(_Integer),
    Float(_Float),
}

pub enum _Integer {
    I16, I32, I64, I128,
}

pub enum _Float {
    F16, F32, F64, F128,
}

impl Expression {
    pub fn new(expression_type: ExpType) -> Expression {
        Expression { expression_type }
    }
}

impl std::fmt::Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.expression_type)
    }
}

impl std::fmt::Display for ExpType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.get_string())
    }
}

impl std::fmt::Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.get_string())
    }
}

impl ExpType {
    pub fn get_string(&self) -> String {
        match self {
            ExpType::Literal(val) => { 
                val.to_string() 
            }
            ExpType::Unary { expression, operator } => {
                format!("{}{}", operator.token_type(), expression.get_string())
            }
            ExpType::Binary { left, operator, right } => {
                format!("{} {} {}", left, operator.token_type(), right)
            }
            ExpType::Grouping { expression } => {
                format!("({})", expression.get_string())
            }
        }
    }
}

impl Literal {
    pub fn new_number(token: &Token) -> Result<Literal, LunalaErrors> {
        let value = token.access_lexeme()?.parse::<i32>();
        match value {
            Ok(value) => Ok(Literal::Number(value)),
            Err(err) => Err(LunalaErrors::new(
                ErrorTypes::LexemeErrorNotANumber(token.clone(), err), 0
            ))
        }
    }
    
    fn get_string(&self) -> String {
        match self {
            Literal::Number(value) => value.to_string(),
            Literal::Bool(value) => value.to_string(),
            Literal::String(value) => value.to_string(),
        }
    }
}
