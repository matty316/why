pub enum Expr {
    Int { value: i32 },
    Str { value: String },
    Bool { value: bool },
}

pub enum Stmt {
    Expr { expr: Expr }
}

pub struct Program {
    pub stmts: Vec<Stmt>
}
