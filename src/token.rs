pub struct Token {
    pub tokenType: TokenType,
    pub literal: String
}

#[derive(Debug, PartialEq)]
pub enum TokenType {
    Plus, Minus, Star, Slash, Eq, EqEq, NotEq, Lt, Gt, LtEq, GtEq
}
