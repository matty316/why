use crate::token::{Token,TokenType};

#[derive(Clone)]
pub struct Scanner {
    input: Vec<u8>,
    prev: usize,
    pos: usize
}

impl Scanner {
    pub fn new(source: String) -> Scanner {
        Scanner { input: source.as_bytes().to_vec(), prev: 0, pos: 0 }
    }

    pub fn scan_token(&mut self) -> Token {
        self.skip_whitespace();

        let c = self.read();

        match c {
            b'+' => Self::make_token(TokenType::Plus, "+"),
            b'-' => Self::make_token(TokenType::Minus, "-"),
            b'*' => Self::make_token(TokenType::Star, "*"),
            b'/' => Self::make_token(TokenType::Slash, "/"),
            b'=' => {
                if self.peek() == b'=' {
                    self.read();
                    Self::make_token(TokenType::EqEq, "==")
                } else {
                    Self::make_token(TokenType::Eq, "=")
                }
            }
            b'<' => {
                if self.peek() == b'=' {
                    self.read();
                    Self::make_token(TokenType::LtEq, "<=")
                } else {
                    Self::make_token(TokenType::Lt, "<")
                }
            }
            b'>' => {
                if self.peek() == b'=' {
                    self.read();
                    Self::make_token(TokenType::GtEq, ">=")
                } else {
                    Self::make_token(TokenType::Gt, ">")
                }
            }
            b'!' => {
                if self.peek() == b'=' {
                    self.read();
                    Self::make_token(TokenType::NotEq, "!=")
                } else {
                    Self::make_token(TokenType::Bang, "!")
                }
            }
            b'"' => self.read_string(),
            0 => Self::make_token(TokenType::Eof, ""),
            _ => {
                if Self::is_alpha(c) {
                    self.read_ident()                    
                } else if Self::is_digit(c) {
                    self.read_num()
                } else {
                    Self::make_token(TokenType::Bang, "!")
                }
            }
        }
    }

    fn skip_whitespace(&mut self) {
        match self.peek() {
            b' ' | b'\n' | b'\t' | b'\r' => { self.read(); }
            _ => (),
        }
    }

    fn peek(&self) -> u8 {
        if self.pos >= self.input.len() {
            return 0;
        }
        self.input[self.pos]
    }

    fn read(&mut self) -> u8 {
        if self.pos >= self.input.len() {
            return 0
        }
        self.prev = self.pos;
        self.pos += 1;
        self.input[self.prev]
    }

    fn make_token(token_type: TokenType, literal: &str) -> Token {
        Token { token_type, literal: literal.to_string() }
    }

    fn is_alpha(c: u8) -> bool {
        b'a' <= c && c <= b'z' || b'A' <= c && c <= b'Z' || c == b'_' 
    }

    fn is_digit(c: u8) -> bool {
        b'0' <= c && c <= b'9'
    }

    fn is_alphanumeric(c: u8) -> bool {
        Self::is_alpha(c) || Self::is_digit(c)
    }

    fn read_ident(&mut self) -> Token {
        let start = self.prev;
        while Self::is_alphanumeric(self.peek()) {
            self.read();
        }
        let bytes = &self.input[start..self.pos];
        Self::check_keyword(bytes)
    }

    fn check_keyword(bytes: &[u8]) -> Token {
        let mut token_type = TokenType::Ident;
        let first = bytes[0];
        match first {
            b'f' => {
                if bytes.len() > 1 {
                    let second = bytes[1];
                    if second == b'u' {
                        token_type = Self::rest(&bytes[2..], &[b'n', b'c'], TokenType::Func)
                    } else if second == b'a' {
                        token_type = Self::rest(&bytes[2..], &[b'l', b's', b'e'], TokenType::False)
                    }
                }
            }
            b'i' => {
                token_type = Self::rest(&bytes[1..], &[b'f'], TokenType::If)
            }
            b'e' => {
                token_type = Self::rest(&bytes[1..], &[b'l', b's', b'e'], TokenType::Else)
            }
            b'v' => {
                token_type = Self::rest(&bytes[1..], &[b'a', b'r'], TokenType::Var)
            }
            b't' => {
                token_type = Self::rest(&bytes[1..], &[b'r', b'u', b'e'], TokenType::True)
            }
            _ => (),
        }

        let string = String::from_utf8_lossy(bytes);
        Self::make_token(token_type, &string)
    }

    fn rest(bytes: &[u8], rest: &[u8], token_type: TokenType) -> TokenType {
        if bytes.len() == rest.len() && bytes == rest {
           return token_type; 
        }
        TokenType::Ident
    }

    fn read_num(&mut self) -> Token {
        let start = self.prev;
        while Self::is_digit(self.peek()) {
            self.read();
        }
        let bytes = &self.input[start..self.pos];
        let string = String::from_utf8_lossy(bytes);
        Self::make_token(TokenType::Num, &string)
    }

    fn read_string(&mut self) -> Token {
        let start = self.prev + 1;

        loop {
            self.read();
            if self.peek() == b'"' || self.peek() == 0 {
                break
            }
        }
        self.read();
        let bytes = &self.input[start..self.prev];
        let string = String::from_utf8_lossy(bytes);
        Self::make_token(TokenType::Str, &string)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scan() {
        let input = "+-*/ = == != < > <= >= ! func var 10 223 5 num1 add _add add_me_up if else \"hell nah\" true false".to_string();

        let exp = vec![
            Scanner::make_token(TokenType::Plus, "+"),
            Scanner::make_token(TokenType::Minus, "-"),
            Scanner::make_token(TokenType::Star, "*"),
            Scanner::make_token(TokenType::Slash, "/"),
            Scanner::make_token(TokenType::Eq, "="),
            Scanner::make_token(TokenType::EqEq, "=="),
            Scanner::make_token(TokenType::NotEq, "!="),
            Scanner::make_token(TokenType::Lt, "<"),
            Scanner::make_token(TokenType::Gt, ">"),
            Scanner::make_token(TokenType::LtEq, "<="),
            Scanner::make_token(TokenType::GtEq, ">="),
            Scanner::make_token(TokenType::Bang, "!"),
            Scanner::make_token(TokenType::Func, "func"),
            Scanner::make_token(TokenType::Var, "var"),
            Scanner::make_token(TokenType::Num, "10"),
            Scanner::make_token(TokenType::Num, "223"),
            Scanner::make_token(TokenType::Num, "5"),
            Scanner::make_token(TokenType::Ident, "num1"),
            Scanner::make_token(TokenType::Ident, "add"),
            Scanner::make_token(TokenType::Ident, "_add"),
            Scanner::make_token(TokenType::Ident, "add_me_up"),
            Scanner::make_token(TokenType::If, "if"),
            Scanner::make_token(TokenType::Else, "else"),
            Scanner::make_token(TokenType::Str, "hell nah"),
            Scanner::make_token(TokenType::True, "true"),
            Scanner::make_token(TokenType::False, "false"),
            Scanner::make_token(TokenType::Eof, ""),
        ];

        let mut s = Scanner::new(input);
        for tok in exp {
            let token = s.scan_token();
            println!("{:?}", token.token_type);
            assert_eq!(tok.token_type, token.token_type);
            assert_eq!(tok.literal, token.literal)
        }
    }
}
