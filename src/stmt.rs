use crate::expr::Expr;

pub trait Stmt {}

pub struct PrintStmt {
    expr: Expr,
}

impl PrintStmt {
    pub fn new(expr: Expr) -> PrintStmt {
        PrintStmt { expr }
    }
}

impl Stmt for PrintStmt {}

pub struct ExprStmt {
    expr: Expr,
}

impl ExprStmt {
    pub fn new(expr: Expr) -> ExprStmt {
        ExprStmt { expr }
    }
}

impl Stmt for ExprStmt {}
