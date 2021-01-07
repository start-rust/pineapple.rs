#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Illegal,
    Blank,
    EOF,
    Let,
    Ident(String),
    LeftParen,
    RightParen,
    Semicolon,
    Assign,
    String(String),
}