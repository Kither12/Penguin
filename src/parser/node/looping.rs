use anyhow::Result;

use crate::environment::environment::Environment;

use super::{expression::Expression, scope::Scope};

#[derive(Debug)]
pub struct WhileLoop<'a> {
    expr: Expression<'a>,
    scope: Scope<'a>,
}

impl<'a> WhileLoop<'a> {
    pub fn new(expr: Expression<'a>, scope: Scope<'a>) -> Self {
        WhileLoop { expr, scope }
    }
    pub fn execute(&'a self, mut environment: Environment<'a>) -> Result<Environment<'a>> {
        let mut expr_val = self.expr.evaluation(&environment)?;
        while expr_val.as_bool()? {
            environment = self.scope.execute(environment)?;
            expr_val = self.expr.evaluation(&environment)?;
        }
        Ok(environment)
    }
}
