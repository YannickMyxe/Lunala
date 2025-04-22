use crate::expressions::ExpType;

pub type Statements = Vec<Statement>;

pub enum Statement {
    Expression(Expression),
    Print(PrintExpression),
}

pub trait StatementTrait {
    fn expression(&self) -> ExpType;
}

pub struct Expression {
    expression: ExpType,
}

pub struct PrintExpression {
    expression: ExpType,
}

impl Statement {
    fn get_string(&self) -> String {
        match self {
            Statement::Expression(expr) => {
                format!("Expression({})", expr)
            }
            Statement::Print(expression) => {
                format!("Print({})", expression)
            }
        }
    }
    
    pub fn new_print(expression: ExpType) -> PrintExpression {
        PrintExpression { expression }
    }
    
    pub fn new_expression(expression: ExpType) -> Expression {
        Expression { expression }
    }
}

impl std::fmt::Display for Statement {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.get_string())
    }
}

impl std::fmt::Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.expression)
    }
}

impl std::fmt::Display for PrintExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.expression)
    }
}

impl StatementTrait for Expression {
    fn expression(&self) -> ExpType {
        self.expression.clone()
    }
}

impl StatementTrait for PrintExpression {
    fn expression(&self) -> ExpType {
        self.expression.clone()
    }
}

impl StatementTrait for Statement {
    fn expression(&self) -> ExpType {
        match self {
            Statement::Expression(e) => {
                e.expression()
            }
            Statement::Print(p) => {
                p.expression()
            }
        }
    }
}