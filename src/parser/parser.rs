use crate::ast::*;
use crate::parser::lexer::Lexer;
use crate::parser::token::Token;

#[derive(Debug)]
pub struct Parse<'a> {
    lexer: Lexer<'a>,
    current_token: Token,
    next_token: Token,
}


impl<'a> Parse<'a> {
    pub fn new(lexer: Lexer<'a>) -> Self {
        let mut parser = Parse {
            lexer,
            current_token: Token::EOF,
            next_token: Token::EOF,
        };
        parser.bump();
        parser.bump();

        parser
    }

    fn bump(&mut self) {
        self.current_token = self.next_token.clone();
        self.next_token = self.lexer.next_token();
    }

    fn current_token_is(&mut self, tok: Token) -> bool {
        self.current_token == tok
    }

    fn next_token_is(&mut self, tok: &Token) -> bool {
        self.next_token == *tok
    }

    fn expect_next_token(&mut self, tok: Token) -> bool {
        return if self.next_token_is(&tok) {
            self.bump();
            true
        } else {
            false
        }
    }

    fn parse_let_stmt(&mut self) -> Option<Stmt> {

        match &self.next_token {
            Token::Ident(_) => self.bump(),
            _ => return None,
        };

        let name = match self.parse_ident() {
            Some(name) => name,
            None => return None,
        };

        if !self.expect_next_token(Token::Assign) {
            return None;
        }

        self.bump();

        let expr = match self.parse_expr(Precedence::Lowest) {
            Some(expr) => expr,
            None => {
                return None
            },
        };

        Some(Stmt::Let(name, expr))
    }

    fn parse_expr(&mut self, precedence: Precedence) -> Option<Expr> {
        let mut left = match self.current_token {
            Token::Ident(_) => self.parse_ident_expr(),
            Token::String(_) => self.parse_string_expr(),
            _ => return None
        };
        while !self.next_token_is(&Token::Semicolon)
            && precedence < self.next_token_precedence() {
            match self.next_token {
                Token::LeftParen => {
                    self.bump();
                    left = self.parse_call_expr(left.unwrap());
                }
                _ => {
                    return left
                }
            }
        }

        left
    }

    fn next_token_precedence(&mut self) -> Precedence {
        Self::token_to_precedence(&self.next_token)
    }

    fn token_to_precedence(tok: &Token) -> Precedence {
        match tok {
            Token::LeftParen => Precedence::Call,
            _ => Precedence::Lowest,
        }
    }

    fn parse_ident_expr(&mut self) -> Option<Expr> {
        match self.parse_ident() {
            Some(ident) => Some(Expr::Ident(ident)),
            None => None,
        }
    }

    fn parse_string_expr(&mut self) -> Option<Expr> {
        match self.current_token {
            Token::String(ref mut s) => Some(Expr::Literal(Literal::String(s.clone()))),
            _ => None,
        }
    }

    fn parse_expr_stmt(&mut self) -> Option<Stmt> {
        match self.parse_expr(Precedence::Lowest) {
            Some(expr) => {
                if self.next_token_is(&Token::Semicolon) {
                    self.bump();
                }
                Some(Stmt::Expr(expr))
            }
            None => None,
        }
    }

    fn parse_stmt(&mut self) -> Option<Stmt> {
        match self.current_token {
            Token::Let => self.parse_let_stmt(),
            _ => self.parse_expr_stmt(),
        }
    }

    fn parse_call_expr(&mut self, func: Expr) -> Option<Expr> {
        let args = match self.parse_expr_list(Token::RightParen) {
            Some(args) => args,
            None => return None,
        };

        Some(Expr::Call {
            func: Box::new(func),
            args,
        })
    }

    fn parse_expr_list(&mut self, end: Token) -> Option<Vec<Expr>> {
        let mut list = vec![];

        if self.next_token_is(&end) {
            self.bump();
            return Some(list);
        }

        self.bump();

        match self.parse_expr(Precedence::Lowest) {
            Some(expr) => list.push(expr),
            None => return None,
        }

        if !self.expect_next_token(end) {
            return None;
        }

        Some(list)
    }

    fn parse_ident(&mut self) -> Option<Ident> {
        match self.current_token {
            Token::Ident(ref mut ident) => Some(Ident(ident.clone())),
            _ => None,
        }
    }

    pub fn parse(&mut self) -> Program {
        let mut program: Program = vec![];

        while !self.current_token_is(Token::EOF) {
            match self.parse_stmt() {
                Some(stmt) => program.push(stmt),
                None => {}
            }
            self.bump();
        }

        program
    }
}

#[cfg(test)]
mod tests {
    use crate::ast::*;
    use crate::parser::lexer::Lexer;
    use crate::parser::parser::Parse;

    #[test]
    fn test_parser() {
        let sr = r#"
let a = "pen pineapple apple pen."
print(a)
"#;
        let mut parser = Parse::new(Lexer::new(sr));
        let program = parser.parse();

        assert_eq!(
            vec![
                Stmt::Let(
                    Ident(String::from("a")),
                    Expr::Literal(Literal::String(String::from("pen pineapple apple pen."))),
                ),
                Stmt::Expr(
                    Expr::Call {
                        func: Box::new(Expr::Ident(Ident(String::from("print")))),
                        args: vec![
                            Expr::Ident(Ident(String::from("a")))
                        ],
                    }
                ),
            ],
            program,
        );
    }
}