use std::collections::HashMap;

#[derive(Clone)]
pub struct Token {
    token_type: TokenType,
    lexeme: Option<String>,
    _line: usize,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: Option<String>, line: usize) -> Token {
        Token { token_type, lexeme, _line: line }
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "[{}", self.token_type)?;
        match self.lexeme {
            Some(ref lexeme) => write!(f, ", {}", lexeme)?,
            None => {}
        }
        write!(f, "]")
    }
}

#[derive(Clone, Debug)]
pub enum TokenType {
    Slash, Comment,

    LeftBracket, RightBracket,
    LeftSquareBracket, RightSquareBracket,
    LeftCurlyBracket, RightCurlyBracket,

    Plus, Minus, Star, Equals, DoubleEquals, Bang, Percent, Colon, Semicolon, Dot,
    LessThan, MoreThan,
    //DoubleQuote,
    SingleQuote, AltQuote,

    String, Number, Identifier,

    And, Or, If, Let,

    Package, Function, Print
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
            TokenType::Percent => {"Percent"}
            TokenType::Colon => {"Colon"}
            TokenType::Semicolon => {"Semicolon"}
            TokenType::Package => {"Package"}
            TokenType::Function => {"Function"}
            TokenType::Print => {"Print"}
            TokenType::LessThan => {"LessThan"}
            TokenType::MoreThan => {"MoreThan"}
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
    }

    pub fn insert(&mut self, key: &str, value: TokenType) {
        self.dict.insert(key.to_owned(), value);
    }

    pub fn get(&self, key: String) -> Option<TokenType> {
        self.dict.get(&key).cloned()
    }
}