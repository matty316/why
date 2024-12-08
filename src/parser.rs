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
        Expr::Int { value: self.current.literal.parse::<i32>().unwrap() }
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
        let source = ["1", "23", "2334523"];
        let exp: Vec<i32> = vec![1, 23, 2334523];

        for (i, s) in source.iter().enumerate() {
            let scanner = Scanner::new(s.to_string());
            let mut p = Parser::new(scanner);
            let program = p.parse();
            let stmt = &program.stmts[0];
            if let Stmt::Expr { expr } = stmt {
                if let Expr::Int { value } = expr {
                    assert_eq!(*value, exp[i])
                }
            }
        }
    }
}
