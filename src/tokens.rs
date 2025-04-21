use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct Token {
    token_type: TokenType,
    lexeme: Option<String>,
    _line: usize,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: Option<String>, line: usize) -> Token {
        Token { token_type, lexeme, _line: line }
    }
    
    pub fn token_type(&self) -> TokenType {
        self.token_type.clone()
    }
    
    pub fn access_lexeme(&self) -> String {
        match self.lexeme.clone() {
            None => { 
                self.token_type.to_string()
            },
            Some(val) => { val }
        }
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "[{} _{}", self.token_type, self._line)?;
        if let Some(ref lexeme) = self.lexeme { write!(f, ", {}", lexeme)? }
        write!(f, "]")
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum TokenType {
    Slash, Comment,

    LeftBracket, RightBracket,
    LeftSquareBracket, RightSquareBracket,
    LeftCurlyBracket, RightCurlyBracket,

    Plus, Minus, Star, Equals, DoubleEquals, Bang, Percent, Colon, Semicolon, Dot,
    LessThan, GreaterThan, LessEquals, GreaterEquals, BangEquals,
    //DoubleQuote,
    SingleQuote, AltQuote,

    String, Number, Identifier,
    True, False,

    And, Or, If, Let,

    Package, Function, Print,
    
    EOF,
}

impl TokenType {
    pub fn map(&self) -> String {
        match self {
            TokenType::Slash => {"Slash"}
            TokenType::Comment => {"Comment"}
            TokenType::LeftBracket => {"LeftBracket"}
            TokenType::RightBracket => {"RightBracket"}
            TokenType::LeftSquareBracket => {"LeftSquareBracket"}
            TokenType::RightSquareBracket => {"RightSquareBracket"}
            TokenType::LeftCurlyBracket => {"LeftCurlyBracket"}
            TokenType::RightCurlyBracket => {"RightCurlyBracket"}
            TokenType::Plus => {"Plus"}
            TokenType::Minus => {"Minus"}
            TokenType::Star => {"Star"}
            TokenType::Equals => {"Equals"}
            TokenType::DoubleEquals => {"DoubleEquals"}
            TokenType::Bang => {"Bang"}
            TokenType::BangEquals => {"BangEquals"}
            TokenType::Percent => {"Percent"}
            TokenType::Colon => {"Colon"}
            TokenType::Semicolon => {"Semicolon"}
            TokenType::Package => {"Package"}
            TokenType::Function => {"Function"}
            TokenType::Print => {"Print"}
            TokenType::LessThan => {"LessThan"}
            TokenType::GreaterThan => {"GreaterThan"}
            TokenType::LessEquals => {"LessThanOrEquals"}
            TokenType::GreaterEquals => {"GreaterThanOrEquals"}
            //TokenType::DoubleQuote => {"DoubleQuote"}
            TokenType::SingleQuote => {"SingleQuote"}
            TokenType::AltQuote => {"AltQuote"}
            TokenType::String => {"String"}
            TokenType::Number => {"Number"}
            TokenType::Dot => {"Dot"}
            TokenType::And => {"And"}
            TokenType::Or => {"Or"}
            TokenType::If => {"If"}
            TokenType::Let => {"Let"}
            TokenType::Identifier => {"Identifier"}
            TokenType::EOF => {"End of File"}
            TokenType::True => {"True"}
            TokenType::False => {"False"}
        }.to_owned()
    }
}

impl std::fmt::Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.map())
    }
}

pub struct ReservedKeywords {
    dict: HashMap<String, TokenType>,
}

impl ReservedKeywords {
    pub fn new() -> ReservedKeywords {
        let mut s = ReservedKeywords { dict: HashMap::new() };
        s.fill();
        s
    }

    fn fill(&mut self) {
        self.insert("and", TokenType::And);
        self.insert("or", TokenType::Or);
        self.insert("if", TokenType::If);
        self.insert("let", TokenType::Let);
        self.insert("fn", TokenType::Function);
        self.insert("package", TokenType::Package);
        self.insert("print", TokenType::Print);
        self.insert("true", TokenType::True);
        self.insert("false", TokenType::False);
    }

    pub fn insert(&mut self, key: &str, value: TokenType) {
        self.dict.insert(key.to_owned(), value);
    }

    pub fn get(&self, key: String) -> Option<TokenType> {
        self.dict.get(&key).cloned()
    }
}