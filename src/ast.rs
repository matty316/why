pub enum Expr {
    Int { value: i32 }
}

pub enum Stmt {
    Expr { expr: Expr }
}

pub struct Program {
    pub stmts: Vec<Stmt>
}
