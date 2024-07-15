use anyhow::{Ok, Result};

use crate::environment::environment::Environment;

use super::{expression::Expression, primitive::PrimitiveType};

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
}

#[derive(Debug)]
pub struct Declaration<'a> {
    identifier: &'a str,
    primitive_type: PrimitiveType,
    expr: Expression<'a>,
}
impl<'a> Declaration<'a> {
    pub fn new(identifier: &'a str, primitive_type: PrimitiveType, expr: Expression<'a>) -> Self {
        Self {
            identifier: identifier,
            primitive_type: primitive_type,
            expr: expr,
        }
    }
    pub fn execute(&self, mut environment: Environment<'a>) -> Result<Environment> {
        let expr_val = self.expr.evaluation(&environment)?;
        environment = environment.subscribe(self.identifier, self.primitive_type, expr_val)?;
        Ok(environment)
    }
}
