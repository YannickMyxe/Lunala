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
}

impl ErrorTypes {
    fn map_error(&self) -> String {
        match self {
            ErrorTypes::InvalidToken(token) => { format!("Invalid token: {}", token) }
            ErrorTypes::UnterminatedString => { "Unterminated string".to_string() }
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