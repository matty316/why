use crate::token::{Token, TokenType};
use crate::scanner::Scanner;
use crate::ast::{Program, Expr, Stmt};

struct Parser {
    scanner: Scanner,
    current: Token,
}

impl Parser {
    fn new(mut scanner: Scanner) -> Parser {
        Parser {
            scanner: scanner.clone(),
            current: scanner.scan_token()
        }
    }

    fn parse(&mut self) -> Program {
        let mut stmts: Vec<Stmt> = vec![];
        while self.current.token_type != TokenType::Eof {
            let stmt = self.parse_stmt();
            stmts.push(stmt);
            self.read();
        }

        Program { stmts }
    }

    fn parse_stmt(&mut self) -> Stmt {
        if self.current.token_type == TokenType::Var { return self.parse_var(); }
        self.parse_expr_stmt()
    }

    fn parse_var(&mut self) -> Stmt {
        self.read();
        self.read();
        let name = self.current.literal.clone();
        self.read();
        self.read();

        let expr = self.parse_expr();
        self.read();

        if self.current.token_type == TokenType::NewLine || self.current.token_type == TokenType::Semicolon {
            self.read();
        }

        Stmt::Var { name, expr }
    }

    fn parse_expr_stmt(&mut self) -> Stmt {
        let expr = self.parse_expr();
        Stmt::Expr { expr }
    }

    fn parse_func(&mut self) -> Expr {
        self.expect(TokenType::Func);
        self.read();
        let name = self.current.literal.clone();
        self.read();
        let params = self.parse_params();
        let body = self.parse_block();
        self.read();
        Expr::Func { name, params, body }
    }

    fn parse_params(&mut self) -> Vec<String> {
        self.expect(TokenType::LParen);
        let mut params = vec![];
        while self.current.token_type != TokenType::RParen {
            let param = self.current.literal.clone();
            params.push(param);
            self.read();
            if self.current.token_type == TokenType::Comma {
                self.read();
            }
        }
        self.expect(TokenType::RParen);

        params
    }

    fn parse_block(&mut self) -> Vec<Stmt> {
        let mut stmts: Vec<Stmt> = vec![];
        self.expect(TokenType::LBrace);
        while self.current.token_type != TokenType::RBrace {
            let stmt = self.parse_stmt();
            stmts.push(stmt);
        }
        stmts
    }

    fn parse_expr(&mut self) -> Expr {
        if self.current.token_type == TokenType::Func {
            return self.parse_func();
        }
        self.parse_term()
    }

    fn parse_term(&mut self) -> Expr {
        let mut left = self.parse_factor();
        self.read();
        while self.current.token_type == TokenType::Plus || self.current.token_type == TokenType::Minus {
            let op = self.current.literal.clone();
            self.read();
            let right = self.parse_factor();
            left = Expr::Binary { left: Box::new(left), right: Box::new(right), op }
        }
        left
    }

    fn parse_factor(&mut self) -> Expr {
        let mut left = self.parse_primary();
        self.read();
        while self.current.token_type == TokenType::Plus || self.current.token_type == TokenType::Minus {
            let op = self.current.literal.clone();
            self.read();
            let right = self.parse_primary();
            left = Expr::Binary { left: Box::new(left), right: Box::new(right), op }
        }
        left
    }

    fn parse_primary(&self) -> Expr {
        match self.current.token_type {
            TokenType::Num => Expr::Int { value: self.current.literal.parse::<i32>().unwrap() },
            TokenType::Str => Expr::Str { value: self.current.literal.clone() },
            TokenType::True => Expr::Bool { value: true },
            TokenType::False => Expr::Bool { value: false },
            TokenType::Ident => Expr::Name { value: self.current.literal.clone() },
            _ => todo!(),
        }
    }

    fn read(&mut self) -> Token {
        let prev = self.current.clone();
        self.current = self.scanner.scan_token();
        prev
    }

    fn expect(&mut self, token: TokenType) {
        if self.current.token_type == token {
            self.read();
            return
        }
        panic!() //TODO: real error handling
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn parse_int() {
        let sources = ["1", "23", "2334523"];
        let exp: Vec<i32> = vec![1, 23, 2334523];

        for (i, s) in sources.iter().enumerate() {
            let program = parse(s);
            let stmt = &program.stmts[0];
            if let Stmt::Expr { expr } = stmt {
                if let Expr::Int { value } = expr {
                    assert_eq!(*value, exp[i])
                }
            }
        }
    }

    #[test]
    fn parse_string() {
        let sources = ["\"hell\"", "\"hell yeah\"", "\"long ahhhh string, not really that long tbh\""];
        let exp = ["hell", "hell yeah", "long ahhhh string, not really that long tbh"];

        for (i, s) in sources.iter().enumerate() {
            let program = parse(s);
            let stmt = &program.stmts[0];
            if let Stmt::Expr { expr } = stmt {
                if let Expr::Str { value } = expr {
                    assert_eq!(value, exp[i])
                }
            }
        }
    }
    
    #[test]
    fn parse_bool() {
        let sources = ["true", "false"];
        let exp = [true, false];

        for (i, s) in sources.iter().enumerate() {
            let program = parse(s);
            let stmt = &program.stmts[0];
            if let Stmt::Expr { expr } = stmt {
                if let Expr::Bool { value } = expr {
                    assert_eq!(*value, exp[i])
                }
            }
        }
    }

    #[test]
    fn parse_var() {
        let source = "var x = 42";
        let program = parse(source);
        let stmt = &program.stmts[0];
        if let Stmt::Var { name, expr } = stmt {
            assert_eq!(name, "x");
            if let Expr::Int { value } = expr {
                assert_eq!(*value, 42);
            }
        }
    }

    #[test]
    fn parse_ident() {
        let source = "foo";
        let program = parse(source);
        let stmt = &program.stmts[0];
        if let Stmt::Expr { expr } = stmt {
            if let Expr::Name { value } = expr {
                assert_eq!(value, "foo");
            }
        }
    }

    #[test]
    fn parse_binary() {
        let source = "1 + 2";
        let program = parse(source);
        let stmt = &program.stmts[0];
        if let Stmt::Expr { expr } = stmt {
            if let Expr::Binary { left, right, op } = expr {
                if let Expr::Int { value } = **left {
                    assert_eq!(value, 1);
                }
                if let Expr::Int { value } = **right {
                    assert_eq!(value, 2);
                }
                assert_eq!(op, "+");
            }
        }
    }

    #[test]
    fn parse_func() {
        let source = "func add(a, b) { a + b }";
        let program = parse(source);
        let stmt = &program.stmts[0];
        if let Stmt::Expr { expr } = stmt {
            if let Expr::Func { name, params, body } = expr {
                assert_eq!(name, "add");
                assert_eq!(params.len(), 2);
                assert_eq!(params[0], "a");
                assert_eq!(params[1], "b");
                if let Stmt::Expr { expr } = &body[0] {
                    if let Expr::Binary { left, right, op } = expr {
                        let left_val = left.clone();
                        let right_val = right.clone();
                        if let Expr::Name { value } = *left_val {
                            assert_eq!(value.clone(), "a");
                        }
                        if let Expr::Name { value } = *right_val {
                            assert_eq!(value.clone(), "b");
                        }
                        assert_eq!(op, "+");
                    } else {
                        panic!();
                    }
                } else {
                    panic!();
                }
            } else {
                panic!();
            }
        } else {
            panic!();
        }
    }

    fn parse(input: &str) -> Program {
        let scanner = Scanner::new(input.to_string());
        let mut p = Parser::new(scanner);
        p.parse()
    }
}
