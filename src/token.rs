#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub literal: String
}

#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    Plus, Minus, Star, Slash, LParen, RParen, LBrace, RBrace, Comma, Eq, EqEq, NotEq, Lt, Gt, LtEq, GtEq, Bang, Num, Str, Ident, Func, Var, If, Else, True, False, NewLine, Semicolon, Eof
}

