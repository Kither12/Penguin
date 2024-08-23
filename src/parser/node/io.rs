use crate::ProgramState;

use super::expression::Expr;
use anyhow::Result;

#[derive(Debug)]
pub struct Output {
    expr: Expr,
    end: String,
}

impl Output {
    pub fn new(expr: Expr, end: String) -> Self {
        Self { expr, end }
    }
    pub fn execute(&self, program: &ProgramState) -> Result<()> {
        let expr_val = self.expr.execute(program)?;
        print!("{}{}", expr_val, self.end);
        Ok(())
    }
}
