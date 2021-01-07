use crate::parser::token::Token;

#[derive(Debug)]
pub struct Lexer<'a> {
    source_code: &'a str,
    pos: usize,
    next_pos: usize,
    ch: u8,
}

impl<'a> Lexer<'a> {
    pub fn new(source_code: &'a str) -> Self {
        let mut lexer = Lexer {
            source_code,
            pos: 0,
            next_pos: 0,
            ch: 0,
        };
        lexer.read_char();
        return lexer;
    }

    fn read_char(&mut self) {
        if self.next_pos >= self.source_code.len() {
            self.ch = 0;
        } else {
            self.ch = self.source_code.as_bytes()[self.next_pos];
        }
        self.pos = self.next_pos;
        self.next_pos += 1
    }

    fn skip_whitespace(&mut self) {
        loop {
            match self.ch {
                b' ' | b'\t' => {
                    self.read_char()
                }
                _ => {
                    break;
                }
            }
        }
    }

    fn consume_string(&mut self) -> Token {
        self.read_char();

        let start_pos = self.pos;

        loop {
            match self.ch {
                b'"' | 0 => {
                    let literal = &self.source_code[start_pos..self.pos];
                    self.read_char();
                    return Token::String(literal.to_string());
                }
                _ => {
                    self.read_char();
                }
            }
        }
    }

    fn consume_identifier(&mut self) -> Token {
        let start_pos = self.pos;

        loop {
            match self.ch {
                b'a'..=b'z' | b'A'..=b'Z' | b'_' => {
                    self.read_char();
                }
                _ => {
                    break;
                }
            }
        }

        let literal = &self.source_code[start_pos..self.pos];

        match literal {
            "let" => Token::Let,
            _ => Token::Ident(String::from(literal)),
        }
    }

    fn nextch(&mut self) -> u8 {
        if self.next_pos >= self.source_code.len() {
            return 0;
        } else {
            return self.source_code.as_bytes()[self.next_pos];
        }
    }

    fn nextch_is(&mut self, ch: u8) -> bool {
        self.nextch() == ch
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let tok = match self.ch {
            b'=' => {
                Token::Assign
            }
            b'(' => {
                Token::LeftParen
            }
            b')' => {
                Token::RightParen
            }
            b'a'..=b'z' | b'A'..=b'Z' | b'_' => {
                return self.consume_identifier();
            }
            b'"' => {
                self.consume_string()
            }
            b'\n' => {
                if self.nextch_is(b'\n') {
                    Token::Blank
                } else {
                    self.read_char();
                    return self.next_token();
                }
            }
            0 => Token::EOF,
            _ => {
                Token::Illegal
            }
        };
        self.read_char();

        return tok;
    }
}

pub fn is_letter(c: char) -> bool {
    c >= 'a' && c <= 'z' || c >= 'A' && c <= 'Z'
}

#[cfg(test)]
mod tests {
    use crate::parser::lexer::Lexer;
    use crate::parser::token::Token;

    #[test]
    fn test_lexer() {
        let sr = r#"
let a = "pen pineapple apple pen."
print(a)
        "#;
        let tests = vec![
            Token::Let,
            Token::Ident(String::from("a")),
            Token::Assign,
            Token::String(String::from("pen pineapple apple pen.")),
            Token::Ident(String::from("print")),
            Token::LeftParen,
            Token::Ident(String::from("a")),
            Token::RightParen,
        ];
        let mut lexer = Lexer::new(sr);

        for expect in tests {
            let tok = lexer.next_token();

            assert_eq!(expect, tok);
        }
    }
}