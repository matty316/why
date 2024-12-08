pub enum Expr {
    Int { value: i32 },
    Str { value: String },
    Bool { value: bool },
    Name { value: String },
    Binary { left: Box<Expr>, right: Box<Expr>, op: String },
}

pub enum Stmt {
    Expr { expr: Expr },
    Var { name: String, expr: Expr },
}

pub struct Program {
    pub stmts: Vec<Stmt>
}
