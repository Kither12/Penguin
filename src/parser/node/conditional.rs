use crate::environment::environment::Environment;

use super::{expression::Expression, scope::Scope};
use anyhow::{Context, Result};

#[derive(Debug)]
pub struct IfElse<'a> {
    if_clause: Vec<(Expression<'a>, Scope<'a>)>,
    else_clause: Option<Scope<'a>>,
}

impl<'a> IfElse<'a> {
    pub fn new(
        if_clause: Vec<(Expression<'a>, Scope<'a>)>,
        else_clause: Option<Scope<'a>>,
    ) -> Self {
        IfElse {
            if_clause,
            else_clause,
        }
    }
    pub fn execute(&'a self, mut environment: Environment<'a>) -> Result<Environment> {
        for (expr, scope) in self.if_clause.iter() {
            let expr_val = expr
                .evaluation(&environment)
                .context("Failed to evaluate expression")?;
            if expr_val.as_bool() {
                environment = scope.execute(environment)?;
                return Ok(environment);
            }
        }
        if let Some(scope) = &self.else_clause {
            environment = scope.execute(environment)?;
        }
        Ok(environment)
    }
}
