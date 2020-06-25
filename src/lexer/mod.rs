use logos::Logos;

#[derive(Logos, Debug, PartialEq, Clone)]
#[allow(non_camel_case_types)]
pub enum Token {
    #[error]
    Error,
    EOF,
    #[regex("[a-zA-Z]+", |lexer| lexer.slice().to_owned())]
    IDENT(String),
    #[regex("[0-9]+", |lexer| lexer.slice().parse())]
    INT(i32),
    #[regex(r#""[^"]*""#, |lexer| lexer.slice()[1..(lexer.slice().len()-1)].to_owned())]
    STRING(String),
    #[token("=")]
    ASSIGN,
    #[token("+")]
    PLUS,
    #[token("-")]
    MINUS,
    #[token("/")]
    SLASH,
    #[token("*")]
    ASTERISK,
    #[token("<")]
    LT,
    #[token(">")]
    GT,
    #[token("!")]
    BANG,
    #[token(",")]
    COMMA,
    #[token(";")]
    SEMICOLON,
    #[token("(")]
    LPAREN,
    #[token(")")]
    RPAREN,
    #[token("{")]
    LBRACE,
    #[token("}")]
    RBRACE,
    #[token("fn")]
    FUNCTION,
    #[token("let")]
    LET,
    #[token("if")]
    IF,
    #[token("else")]
    ELSE,
    #[token("return")]
    RETURN,
    #[token("true")]
    TRUE,
    #[token("false")]
    FALSE,
    #[token("==")]
    EQ,
    #[token("!=")]
    NOT_EQ,
}

pub fn lex(input: &str) -> Vec<Token> {
    let mut tokens = Token::lexer(input).collect::<Vec<Token>>();
    tokens.push(Token::EOF);
    tokens
}
