use crate::environment::environment::Environment;

use super::expression::ExpressionPool;
use anyhow::Result;

#[derive(Debug)]
pub struct Output<'a> {
    expr_pool: ExpressionPool<'a>,
    end: &'a str,
}

impl<'a> Output<'a> {
    pub fn new(expr_pool: ExpressionPool<'a>, end: &'a str) -> Self {
        Self { expr_pool, end }
    }
    pub fn execute(&'a self, environment: &'a Environment<'a>) -> Result<()> {
        let expr_val = self.expr_pool.execute(environment)?;
        print!("{}{}", expr_val, self.end);
        Ok(())
    }
}
