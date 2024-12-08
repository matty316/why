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
        let stmt = self.parse_stmt();
        stmts.push(stmt);
        Program { stmts }
    }

    fn parse_stmt(&mut self) -> Stmt {
        self.parse_expr_stmt()
    }

    fn parse_expr_stmt(&mut self) -> Stmt {
        let expr = self.parse_expr();
        Stmt::Expr { expr }
    }

    fn parse_expr(&mut self) -> Expr {
        self.parse_primary()   
    }

    fn parse_primary(&self) -> Expr {
        match self.current.token_type {
            TokenType::Num => Expr::Int { value: self.current.literal.parse::<i32>().unwrap() },
            TokenType::Str => Expr::Str { value: self.current.literal.clone() },
            TokenType::True => Expr::Bool { value: true },
            TokenType::False => Expr::Bool { value: false },
            _ => todo!(),
        }
    }

    fn read(&mut self) -> Token {
        let prev = self.current.clone();
        self.current = self.scanner.scan_token();
        prev
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

    fn parse(input: &str) -> Program {
        let scanner = Scanner::new(input.to_string());
        let mut p = Parser::new(scanner);
        p.parse()
    }
}
