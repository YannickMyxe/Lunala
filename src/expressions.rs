
pub enum Expression {
    Literal(Literal),
    Unary(UnaryOperator, Expression),
    Binary(Expression, Operator, Expression),
    Group(Expression),
}

pub enum Literal {
    String(String),
    Boolean(bool),
    Number
}

pub enum Number {
    Integer(i64),
    Float(f64),
}

pub enum Boolean {
    True, False
}

pub enum UnaryOperator {
    Negate,
    Bang,
}

pub enum Operator {
    Plus,
    Minus,
    Times,
}

