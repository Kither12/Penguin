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
    pub fn execute(&'a self, environment: Environment<'a>) -> Result<Environment<'a>> {
        let (mut env, mut expr_val) = self.expr.execute(environment)?;
        while expr_val.as_bool()? {
            env = self.scope.execute(env)?;
            (env, expr_val) = self.expr.execute(env)?;
        }
        Ok(env)
    }
}
