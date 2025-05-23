use crate::errors::{ErrorTypes, LunalaErrors};
use crate::expressions::ExpType::{Binary, Grouping, Unary};
use crate::expressions::{ExpType, Literal};
use crate::statement;
use crate::statement::Statement;
use crate::tokens::{Token, TokenType};

pub struct Parser {
    tokens: Vec<Token>,
    cursor: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser { tokens, cursor: 0 }
    }

    pub fn _get_tokens(&self) -> Vec<Token> {
        self.tokens.clone()
    }
    
    pub fn parse(&mut self) -> Result<statement::Statements, LunalaErrors> {
        let mut statements = Vec::new();
        while !self.at_end()? {
            statements.push(self.statement()?);
        }
        Ok(statements)
    }

    fn statement(&mut self) -> Result<Statement, LunalaErrors> {
        match self.peek()?.token_type() {
            TokenType::Print => {
                self.advance()?;
                self.print_statement()
            }
            _ => self.expression_statement()
        }
    }

    fn print_statement(&mut self) -> Result<Statement, LunalaErrors> {
        let expression = self.expression()?;
        self.consume(TokenType::Semicolon, "Expected a `;` after value.")?;
        Ok(Statement::Print(Statement::new_print(expression)))
    }
    
    fn expression_statement(&mut self) -> Result<Statement, LunalaErrors> {
        let expression = self.expression()?;
        self.consume(TokenType::Semicolon, "Expected a `;` after value.")?;
        Ok(Statement::Expression(Statement::new_expression(expression)))
    }

    fn expression(&mut self) -> Result<ExpType, LunalaErrors> {
        self.equality()
    }

    fn equality(&mut self) -> Result<ExpType, LunalaErrors> {
        let mut expression = self.comparison();

        match self.peek()?.token_type() {
            TokenType::BangEquals | TokenType::DoubleEquals => {
                self.advance()?;
                let operator = self.previous()?.clone();
                let right = Box::from(self.comparison()?);
                //println!("Equality: [{}, {}]", operator, right);
                expression = Ok(Binary {
                    operator, right,
                    left: Box::from(expression?),
                })
            }
            _ => {  }
        }
        expression
    }

    fn comparison(&mut self) -> Result<ExpType, LunalaErrors> {
        let mut expression = self.term();

        match self.peek()?.token_type() {
            TokenType::LessThan | TokenType::LessEquals | TokenType::GreaterThan | TokenType::GreaterEquals => {
                self.advance()?;
                let operator = self.previous()?.clone();
                let right = Box::from(self.term()?);
                expression = Ok(Binary {
                    operator, right,
                    left: Box::from(expression?),
                })
            }
            _ => {  }
        }
        expression
    }

    fn term(&mut self) -> Result<ExpType, LunalaErrors> {
        let mut expression = self.factor()?;

        while matches!(self.peek()?.token_type(), TokenType::Plus | TokenType::Minus) {
            self.advance()?;
            let operator = self.previous()?.clone();
            let right = Box::from(self.factor()?);
            expression = Binary {
                operator,
                right,
                left: Box::from(expression),
            };
        }

        Ok(expression)
    }

    fn factor(&mut self) -> Result<ExpType, LunalaErrors> {
        let mut expression = self.unary()?;
    
        while matches!(self.peek()?.token_type(), TokenType::Slash | TokenType::Star) {
            self.advance()?;
            let operator = self.previous()?.clone();
            let right = Box::from(self.unary()?);
            expression = Binary {
                operator,
                right,
                left: Box::from(expression),
            };
        }
    
        Ok(expression)
    }

    fn unary(&mut self) -> Result<ExpType, LunalaErrors> {
        match self.peek()?.token_type() {
            TokenType::Minus | TokenType::Bang => {
                self.advance()?;
                let operator = self.previous()?.clone();
                //println!("Unary: [{}, {}]", operator.clone(), self.peek()?);
                Ok(Unary {operator, expression: Box::from(self.unary()?) })
            },
            _ => {
                self.primary()
            }
        }
    }

    fn primary(&mut self) -> Result<ExpType, LunalaErrors> {
        //println!("Literal [{}]", self.peek()?);
        let expression = match self.peek()?.token_type() {
            TokenType::True => {
                self.advance()?;
                ExpType::Literal(Literal::Bool(true)) },
            TokenType::False => { 
                self.advance()?; 
                ExpType::Literal(Literal::Bool(false)) },
            TokenType::Number => {
                self.advance()?;
                ExpType::Literal(Literal::new_number(self.previous()?)?)
            },
            TokenType::String => {
                self.advance()?;
                ExpType::Literal(Literal::String(self.previous()?.access_lexeme()))
            },
            TokenType::LeftBracket => {
                self.advance()?;
                let expression = self.expression()?;
                self.consume(TokenType::RightBracket, "Expected a `)` after expression.")?;
                //println!("Group: [{}]", expression);
                Grouping { expression: Box::from(expression), }
            },
            _ => { return Err(LunalaErrors::new(ErrorTypes::ExpressionExpected(self.peek()?.to_string()), self.cursor)) }
        };
        Ok(expression)
    }

    fn peek(&self) -> Result<&Token, LunalaErrors> {
        match self.tokens.get(self.cursor) {
            Some(token) => { 
                //println!("Peek: {}", token);
                Ok(token) 
            },
            None => Err(LunalaErrors::new(ErrorTypes::InvalidToken(format!("No token at cursor[{}]", self.cursor)), self.cursor))
        }
    }

    fn check(&self, token_type: TokenType) -> Result<bool, LunalaErrors> {
        if self.at_end()? {
            return Ok(false)
        }
        Ok(self.peek()?.token_type() == token_type)
    }

    fn at_end(&self) -> Result<bool, LunalaErrors> {
        Ok(self.peek()?.token_type() == TokenType::EOF)
    }

    fn advance(&mut self) -> Result<&Token, LunalaErrors> {
        if !self.at_end()? {
            self.cursor += 1;
        }
        self.previous()
    }

    fn previous(&mut self) -> Result<&Token, LunalaErrors> {
        if self.cursor == 0 {
            return Err(LunalaErrors::new(ErrorTypes::NoPreviousItem(0), self.cursor));
        }
        
        match self.tokens.get(self.cursor - 1) {
            None => {
                Err(LunalaErrors::new(ErrorTypes::NoPreviousItem(self.cursor - 1), self.cursor))
            }
            Some(token) => {
                Ok(token)
            }
        }
    }
    
    fn consume(&mut self, token_type: TokenType, message: &str,) -> Result<&Token, LunalaErrors> {
        if self.check(token_type.clone())? {
           return self.advance()
        }
        Err(LunalaErrors::new(ErrorTypes::Error(
            format!("Consume of `{}` to {} failed `{}`", token_type, self.peek()?, message),
        ), self.cursor))
    }
    
    
    fn _sync(&mut self) -> Result<(), LunalaErrors> {
        self.advance()?;
        
        while !self.at_end()? {
            
            if self.previous()?.token_type() == TokenType::Semicolon { return Ok(()) }
            
            match self.peek()?.token_type() {
                TokenType::Function => {
                    return Ok(());
                },
                _ => {
                    //
                },
            }
            
            self.advance()?;
        }
        
        todo!()
    }

}