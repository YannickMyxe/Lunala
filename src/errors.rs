use crate::tokens::Token;
use crate::tree::ExpType;
use std::num::ParseFloatError;

pub struct LunalaErrors {
    e_type: ErrorTypes,
    line: usize,
}

impl LunalaErrors {
    pub fn new(e_type: ErrorTypes, line: usize) -> LunalaErrors {
        LunalaErrors { e_type, line }
    }
}

pub enum ErrorTypes {
    InvalidToken(String),
    UnterminatedString,
    NoPreviousItem(usize),
    ErrorNotANumber(String, Option<ParseFloatError>),
    Error(String),
    ExpressionExpected(String),
    NotABooleanValue(String),
    InvalidUnaryExpression(Token, ExpType),
}

impl ErrorTypes {
    fn map_error(&self) -> String {
        match self {
            ErrorTypes::InvalidToken(token) => { format!("Invalid token: `{}`", token) }
            ErrorTypes::UnterminatedString => { "Unterminated string".to_string() }
            ErrorTypes::NoPreviousItem(location) => { format!("No previous item found at location: {}", location) }
            ErrorTypes::ErrorNotANumber(token, err) => {
                let err_msg = match err {
                    None => {"".to_string()}
                    Some(err) => {
                        format!(", reason -> {}", err)
                    }
                };
                format!("Cannot convert `{}` to a number{}", token, err_msg)
            }
            ErrorTypes::Error(message) => { format!("Error occurred: {}", message) }
            ErrorTypes::ExpressionExpected(message) => { format!("Expected an expression, got: {}", message) }            
            ErrorTypes::NotABooleanValue(val) => {
                format!("Expected a boolean, found {}", val)
            }
            ErrorTypes::InvalidUnaryExpression(operator, exp) => {
                format!("Invalid unary expression: [{}, {}]", operator, exp)
            }
        }.to_owned()
    }
}

impl std::fmt::Debug for ErrorTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.map_error())
    }
}

impl std::fmt::Display for ErrorTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.map_error())
    }
}

impl std::fmt::Display for LunalaErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} at line {}", self.e_type.map_error(), self.line )
    }
}

impl std::fmt::Debug for LunalaErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} at line {}", self.e_type.map_error(), self.line )
    }
}