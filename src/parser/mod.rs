#[derive(Debug, PartialEq, Clone)]
pub enum Statement {
    Let { name: String, value: Expr },
    Return { value: Expr},
    Expression(Expr)
}

#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    Const(i32),
    String(String),
    Boolean(bool),
    Ident(String),
    Prefix { prefix: Prefix, value: Box<Expr> },
    Infix { left: Box<Expr>, infix: Infix, right: Box<Expr> },
    If { condition: Box<Expr>, consequence: Vec<Statement>, alternative: Vec<Statement> },
    Function { parameters: Vec<String>, body: Vec<Statement> },
    Call { function: Box<Expr>, args: Vec<Expr> },
}

#[derive(Debug, PartialEq, Clone)]
pub enum Prefix {
    Bang,
    Minus,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Infix {
    Plus,
    Minus,
    Multiply,
    Divide,
    GreaterThan,
    LessThan,
    Equals,
    NotEquals,
}
