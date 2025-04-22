use crate::errors::{ErrorTypes, LunalaErrors};
use crate::tokens::TokenType;
use crate::expressions::{ExpType, Expression, Literal};

pub struct Interpreter {}

pub type Object = Literal;

impl Object {
    fn from_literal(literal: Literal) -> Object {
        match literal {
            Literal::Number(number) => {
                Object::Number(number)
            }
            Literal::Bool(bool) => {
                Object::Bool(bool)
            }
            Literal::String(string) => {
                Object::String(string)
            }
        }
    }
}

impl Interpreter {
    fn visit_expression(expression: &ExpType) -> Result<Object, LunalaErrors> {
        match expression {
            ExpType::Literal(literal) => {
                //println!("Literal: [{}]", literal);
                Ok(Object::from_literal(literal.clone()))
            }
            ExpType::Unary { operator, expression } => {
                println!("Unary: [{}, {}]", operator, expression);
                match (operator.token_type(), *expression.clone()) {
                    (TokenType::Minus, ExpType::Literal(literal)) => {
                        let number = - literal._get_number()?;
                        Ok(Object::Number(number))
                    },
                    (TokenType::Bang, exp) => {
                        let bool = ! Self::truthy(exp.clone())?;
                        Ok(Object::Bool(bool))
                    }
                    _ => Err(LunalaErrors::new(ErrorTypes::InvalidUnaryExpression(operator.clone(), *expression.clone()), 0))
                }

            }
            ExpType::Binary { operator, left, right } => {
                //println!("Binary: [{}, {}, {}]", left, operator, right);
                
                let left = Self::visit_expression(left)?;
                let right = Self::visit_expression(right)?;
                
                match operator.token_type() {
                    TokenType::Plus => {
                        Ok(Object::Number(left._get_number()? + right._get_number()?))
                    },
                    TokenType::Minus => {
                        Ok(Object::Number(left._get_number()? - right._get_number()?))
                    },
                    TokenType::Slash => {
                        Ok(Object::Number(left._get_number()? / right._get_number()?))
                    },
                    TokenType::Star => {
                        Ok(Object::Number(left._get_number()? * right._get_number()?))
                    },
                    TokenType::GreaterThan => {
                        Ok(Object::Bool(left._get_number()? > right._get_number()?))
                    },
                    TokenType::GreaterEquals => {
                        Ok(Object::Bool(left._get_number()? >= right._get_number()?))
                    },
                    TokenType::LessThan => {
                        Ok(Object::Bool(left._get_number()? < right._get_number()?))
                    },
                    TokenType::LessEquals => {
                        Ok(Object::Bool(left._get_number()? <= right._get_number()?))
                    },
                    TokenType::BangEquals => {
                        Ok(Object::Bool(! Self::equal(&left, &right)))
                    },
                    TokenType::DoubleEquals => {
                        Ok(Object::Bool( Self::equal(&left, &right)))
                    },
                    _ => Err(LunalaErrors::new(ErrorTypes::Error("Not a binary operator".to_owned()), 0))
                }
            }
            ExpType::Grouping { expression } => {
                //println!("Group: {}", expression);
                Self::visit_expression(expression)
            }
        }
    }

    fn truthy(exp: ExpType) -> Result<bool, LunalaErrors> {
        match exp {
            ExpType::Literal(Literal::Bool(val)) => { Ok(val) },
            _ => Err(LunalaErrors::new(ErrorTypes::NotABooleanValue(exp.to_string()), 0))
        }
    }

    fn equal(left: &Object, right: &Object) -> bool {
        match (left, right) {
            (Object::Bool(l), Object::Bool(r)) => { l == r }
            (Object::Number(l), Object::Number(r)) => { l == r}
            (Object::String(l), Object::String(r)) => l == r,
            (_, _) => {
                false
            }
        }
    }

    pub fn interpret(expression: Expression) -> Result<Object, LunalaErrors> {
         Self::visit_expression(&expression._get_type())
    }
}