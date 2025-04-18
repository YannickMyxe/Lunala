

pub struct Tree {
    root: Node,
}

pub enum Node {
    Literal(Literal),
    Unary {
        operator: Operator,
        expression: Box<Node>,
    },
    Binary {
        operator: Operator,
        left: Box<Node>,
        right: Box<Node>,
    },
}

pub enum Operator {
    Plus, Minus,
    Times, Divide,
    Bang,
}

pub enum Literal {
    Number(Number),
    String(String),
}

pub enum Number {
    Int(Integer),
    Float(Float),
}

pub enum Integer {
    I16, I32, I64, I128,
}

pub enum Float {
    F16, F32, F64, F128,
}

impl Tree {
    pub fn new(root: Node) -> Self {
        Self { root }
    }
}

impl Node {

}
