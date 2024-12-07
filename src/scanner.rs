use crate::token::{Token,TokenType};

struct Scanner {
    input: String,
    pos: usize
}

impl Scanner {
    fn new(input: String) -> Scanner {
        Scanner { input: input, pos: 0 }
    }

    fn scan_token(&mut self) -> Token {
        let bytes = self.input.as_bytes();

        let c = bytes[self.pos];

        self.pos += 1;

        match c {
            b'+' => return Self::make_token(TokenType::Plus, "+"),
            b'-' => return Self::make_token(TokenType::Minus, "-"),
            b'*' => return Self::make_token(TokenType::Star, "*"),
            b'/' => return Self::make_token(TokenType::Slash, "/"),
            b'=' => return Self::make_token(TokenType::Eq, "="),
            _ => {
                todo!()
            }
        }
    }

    fn make_token(tokenType: TokenType, literal: &str) -> Token {
        Token { tokenType: tokenType, literal: literal.to_string() }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scan_symbols() {
        let input = "+-*/ = == != < > <= >=".to_string();

        let exp = vec![
            Scanner::make_token(TokenType::Plus, "+"),
        ];

        let mut s = Scanner::new(input);
        for tok in exp {
            let token = s.scan_token();
            assert_eq!(tok.tokenType, token.tokenType);
            assert_eq!(tok.literal, token.literal)
        }
    }
}
