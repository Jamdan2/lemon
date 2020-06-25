use crate::lexer::Token;

#[derive(Debug, PartialEq, Clone)]

pub enum Expr {
    Const(i32),
    String(String),
    Boolean(bool),
}

pub enum Statement {
    Let { name: String, value: Expr },
    Return { value: Expr },
    Expression(Expr),
}

fn parse(input: &mut Vec<Token>) -> Vec<Statement> {
    let mut program = vec![];

    loop {
        let token = &input[0];

        match token {
            Token::EOF => break,
        }
    }

    program
}

fn parse_expr(input: &mut Vec<Token>) -> Expr {
    let left_expr = match input.remove(0) {
        Token::INT(value) => Expr::Const(value),
        Token::TRUE => Expr::Boolean(true),
        Token::FALSE => Expr::Boolean(false),
        Token::
    };
}
