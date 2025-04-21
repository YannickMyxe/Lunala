use crate::errors::{ErrorTypes, LunalaErrors};
use crate::tokens::{ReservedKeywords, Token, TokenType};

pub struct Scanner {
    source: Vec<char>,
    tokens: Vec<Token>,
    cursor: usize,
}

impl Scanner {
    pub fn new(source: &str) -> Scanner {
        let mut s = source.chars().collect::<Vec<char>>();
        s.insert(0, ' ');
        Scanner { source: s, tokens: Vec::new(), cursor: 0 }
    }

    pub fn current(&self) -> Option<&char> {
        self.source.get(self.cursor)
    }
    
    pub fn advance(&mut self) -> Option<&char> {
        self.cursor += 1;
        self.current()
    }
    
    pub fn peek(&self) -> Option<&char> {
       self.source.get(self.cursor + 1)
    }
    
    fn _pop(&self) -> Option<&char> {
        self.source.get(self.cursor)
    }
    
    pub fn at_end(&self) -> bool {
        self.cursor >= self.source.len()
    }

    fn add_token(&mut self, token: Token) {
        println!("[{}] Added token: {}", self.tokens.len(), token,);
        self.tokens.push(token);
    }

    fn add(&mut self, token_type: TokenType) {
        self.add_token(Token::new(token_type, None, self.cursor));
    }

    pub fn scan_tokens(&mut self) -> Result<Vec<Token>, LunalaErrors> {
        while !self.at_end() {
            let current_char = match self.advance().cloned() {
                Some(c) => c,
                None => break,
            };
            let current_token_binding = current_char;
            
            if current_char.is_whitespace() { continue; }

            //println!("c[{}]", current_char);
            
            if current_char.is_numeric() { self.number(); continue; }
            if current_char.is_alphabetic() { self.alpha(); continue; }
            
            match (current_char, self.peek()) {
                ('/', _) => {
                    match self.peek() {
                        Some('/') => {
                                self.add(TokenType::Comment);
                                while self.peek() != Some(&'\n') && !self.at_end() { let _ = self.advance(); }
                        },
                        None | Some(_) => {
                            self.add(TokenType::Slash)
                        }
                    }
                },
                ('*', _) => { self.add(TokenType::Star) },
                ('=', Some('=')) => {
                    self.advance();
                    self.add(TokenType::DoubleEquals);
                },
                ('=', _) => { self.add(TokenType::Equals) },
                ('<', Some('=')) => {
                    self.advance();
                    self.add(TokenType::LessEquals);
                }
                ('<', _) => { self.add(TokenType::LessThan) },
                ('>', Some('=')) => {
                    self.advance();
                    self.add(TokenType::GreaterEquals);
                }
                ('>', _) => { self.add(TokenType::GreaterThan) },
                ('!', Some('=') ) => {
                    self.advance();
                    self.add(TokenType::BangEquals);
                }
                ('!', _) => { self.add(TokenType::Bang) },
                ('"', _) => {
                    let start = self.cursor;
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
                (';', _) => { self.add(TokenType::Semicolon) },
                ('+', _) => { self.add(TokenType::Plus)}
                ('-', _) => { self.add(TokenType::Minus) },
                ('.', _) => { self.add(TokenType::Dot) },
                ('{', _) => { self.add(TokenType::LeftCurlyBracket) },
                ('}', _) => { self.add(TokenType::RightCurlyBracket) },
                ('[', _) => { self.add(TokenType::LeftSquareBracket) },
                (']', _) => { self.add(TokenType::RightSquareBracket) },
                ('(', _) => { self.add(TokenType::LeftBracket) },
                (')', _) => { self.add(TokenType::RightBracket) },
                ('%', _) => { self.add(TokenType::Percent) },
                (':', _) => { self.add(TokenType::Colon) },
                ('\'', _) => { self.add(TokenType::SingleQuote) },
                ('`', _) => { self.add(TokenType::AltQuote) }
                _ => { return self.error(ErrorTypes::InvalidToken(current_token_binding.to_string())); },
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
        let start = self.cursor;
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
        let value: String = self.source[start..self.cursor+1].iter().collect();
        self.add_token(Token::new(TokenType::Number, Some(value), self.cursor));
    }

    pub fn is_alpha_numeric(&self, character: Option<&char>) -> bool {
        match character {
            Some(c) => c.is_alphanumeric(),
            None => false,
        }
    }

    pub fn alpha(&mut self) {
        let start = self.cursor;
        while self.is_alpha_numeric(self.peek()) {
            let _ = self.advance();
        }
        let keywords = ReservedKeywords::new();
        let value: String = self.source[start..self.cursor+1].iter().collect();
        match keywords.get(value.clone()) {
            Some(token_type) => {
                self.add_token(Token::new(token_type.clone(), None, self.cursor));
            }
            None => {
                self.add_token(Token::new(TokenType::Identifier, Some(value), self.cursor));
            }
        }
    }
    
    pub fn _get_tokens(&self) -> Vec<Token> {
        self.tokens.clone()
    }

}