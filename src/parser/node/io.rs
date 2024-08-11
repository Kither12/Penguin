use crate::environment::environment::Environment;

use super::expression::Expression;
use anyhow::Result;

#[derive(Debug)]
pub struct Output<'a> {
    expr: Expression<'a>,
    end: &'a str,
}

impl<'a> Output<'a> {
    pub fn new(expr: Expression<'a>, end: &'a str) -> Self {
        Self { expr, end }
    }
    pub fn execute(&'a self, mut environment: Environment<'a>) -> Result<Environment> {
        let expr_val;
        (environment, expr_val) = self.expr.execute(environment)?;
        print!("{}{}", expr_val, self.end);
        Ok(environment)
    }
}
