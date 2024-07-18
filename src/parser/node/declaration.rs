use anyhow::{Context, Ok, Result};

use crate::environment::environment::Environment;

use super::expression::Expression;

#[derive(Debug)]
pub struct Assignment<'a> {
    identifier: &'a str,
    expr: Expression<'a>,
}

impl<'a> Assignment<'a> {
    pub fn new(identifier: &'a str, expr: Expression<'a>) -> Self {
        Self {
            identifier: identifier,
            expr: expr,
        }
    }
    pub fn execute(&self, mut environment: Environment<'a>) -> Result<Environment> {
        let expr_val = self.expr.evaluation(&environment)?;
        environment = environment
            .assign_var(self.identifier, expr_val)
            .context(format!("Failed to assign {}", self.identifier))?;
        Ok(environment)
    }
}

#[derive(Debug)]
pub struct Declaration<'a> {
    identifier: &'a str,
    expr: Expression<'a>,
}
impl<'a> Declaration<'a> {
    pub fn new(identifier: &'a str, expr: Expression<'a>) -> Self {
        Self {
            identifier: identifier,
            expr: expr,
        }
    }
    pub fn execute(&self, mut environment: Environment<'a>) -> Result<Environment> {
        let expr_val = self.expr.evaluation(&environment)?;
        environment = environment.subscribe(self.identifier, expr_val)?;
        Ok(environment)
    }
}
