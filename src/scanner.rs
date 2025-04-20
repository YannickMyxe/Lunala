use crate::tokens::{ReservedKeywords, Token, TokenType};
use crate::errors;
use crate::errors::{ErrorTypes, LunalaErrors};

pub struct Scanner {
    source: Vec<char>,
    tokens: Vec<Token>,
    cursor: usize,
}

impl Scanner {
    pub fn new(source: &str) -> Scanner {
        Scanner { source: source.chars().collect(), tokens: Vec::new(), cursor: 0 }
    }

    pub fn current(&self) -> Option<&char> {
        self.source.get(self.cursor)
    }
    
    pub fn advance(&mut self) -> Option<&char> {
        self.cursor += 1;
        self.current()
    }
    
    pub fn peek(&self) -> Option<&char> {
        self.source.get(self.cursor)
    }
    
    fn _pop(&self) -> Option<&char> {
        self.source.get(self.cursor)
    }
    
    pub fn at_end(&self) -> bool {
        self.cursor >= self.source.len()
    }

    fn add_token(&mut self, token: Token) {
        println!("Added token: {}", token);
        self.tokens.push(token);
    }

    fn add(&mut self, token_type: TokenType) {
        self.add_token(Token::new(token_type, None, self.cursor));
    }

    pub fn scan_tokens(&mut self) -> Result<Vec<Token>, errors::LunalaErrors> {
        while !self.at_end() {
            let current_char = match self.advance(){
                Some(c) => c,
                None => break,
            };
            let current_token_binding = current_char.clone();
            if current_char.is_ascii_whitespace() { continue; }
            if current_char.is_numeric() { self.number(); self.cursor-=1; continue; }
            if current_char.is_alphabetic() { self.alpha(); self.cursor-=1; continue; }
            println!("c[{}]", current_token_binding);
            match current_char {
                '/' => {
                    match self.peek() {
                        Some(next_char) => match next_char {
                            '/' => {
                                self.add(TokenType::Comment);
                                while self.peek() != Some(&'\n') && !self.at_end() { let _ = self.advance(); }
                            }
                            &_ => {
                                self.add(TokenType::Slash)
                            }
                        },
                        None => {
                            self.add(TokenType::Slash)
                        }
                    }
                },
                '*' => { self.add(TokenType::Star) },
                '=' => {
                    match self.peek() {
                        Some(next_char) => match next_char {
                            '=' => {
                                self.add(TokenType::DoubleEquals);
                            }
                            &_ => {
                                self.add(TokenType::Equals)
                            }
                        },
                        None => {
                            self.add(TokenType::Equals)
                        }
                    }
                },
                '<' => { self.add(TokenType::LessThan)},
                '>' => { self.add(TokenType::MoreThan)},
                '!' => { self.add(TokenType::Bang)},
                '"' => {
                    let start = self.cursor.clone();
                    let _ = self.advance();
                    while self.peek() != Some(&'"') && !self.at_end() {
                        let _ = self.advance();
                    }
                    if self.at_end() {
                        return self.error(ErrorTypes::UnterminatedString);
                    }
                    let _ = self.advance();
                    let value: String = self.source[start..self.cursor-1].iter().collect();
                    self.add_token(Token::new(TokenType::String, Some(value), self.cursor));
                    self.cursor-=1;
                },
                ';' => { self.add(TokenType::Semicolon) },
                '+' => { self.add(TokenType::Plus)},
                '.' => { self.add(TokenType::Dot) },
                '{' => { self.add(TokenType::LeftCurlyBracket) },
                '}' => { self.add(TokenType::RightCurlyBracket) },
                '[' => { self.add(TokenType::LeftSquareBracket) },
                ']' => { self.add(TokenType::RightSquareBracket) },
                '(' => { self.add(TokenType::LeftBracket) },
                ')' => { self.add(TokenType::RightBracket) },
                '-' => { self.add(TokenType::Minus) },
                '%' => { self.add(TokenType::Percent) },
                ':' => { self.add(TokenType::Colon) },
                '\'' => { self.add(TokenType::SingleQuote) },
                '`' => { self.add(TokenType::AltQuote) }
                &_ => { return self.error(ErrorTypes::InvalidToken(current_token_binding.to_string())); },
            };
        }
        self.add(TokenType::EOF);
        Ok(self.tokens.clone())
    }

    pub fn error(&self, error_types: ErrorTypes) -> Result<Vec<Token>, LunalaErrors> {
        Err(LunalaErrors::new(error_types, self.cursor))
    }

    pub fn is_digit(&self, character: Option<&char>) -> bool {
        match character {
            Some(c) => c.is_ascii_digit(),
            None => false,
        }
    }

    pub fn number(&mut self) {
        let start = self.cursor.clone();
        while self.is_digit(self.peek()) {
            let _ = self.advance();
        }
        if self.peek() == Some(&'.') {
            // Consume the dot
            let _ = self.advance();
            while self.is_digit(self.peek()) {
                let _ = self.advance();
            }
        }
        let value: String = self.source[start..self.cursor].iter().collect();
        self.add_token(Token::new(TokenType::Number, Some(value), self.cursor));
    }

    pub fn is_alpha_numeric(&self, character: Option<&char>) -> bool {
        match character {
            Some(c) => c.is_alphanumeric(),
            None => false,
        }
    }

    pub fn alpha(&mut self) {
        let start = self.cursor.clone();
        while self.is_alpha_numeric(self.peek()) {
            let _ = self.advance();
        }
        let keywords = ReservedKeywords::new();
        let value: String = self.source[start..self.cursor].iter().collect();
        match keywords.get(value.clone()) {
            Some(token_type) => {
                self.add_token(Token::new(token_type.clone(), None, self.cursor));
            }
            None => {
                self.add_token(Token::new(TokenType::Identifier, Some(value), self.cursor));
            }
        }
    }

}