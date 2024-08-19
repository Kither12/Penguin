use crate::environment::environment::Environment;

use super::{
    expression::ExpressionPool,
    scope::{FlowStatement, Scope},
};
use anyhow::{Context, Result};

#[derive(Debug)]
pub struct IfElse<'a> {
    if_clause: Box<[(ExpressionPool<'a>, Scope<'a>)]>,
    else_clause: Option<Scope<'a>>,
}

impl<'a> IfElse<'a> {
    pub fn new(
        if_clause: Box<[(ExpressionPool<'a>, Scope<'a>)]>,
        else_clause: Option<Scope<'a>>,
    ) -> Self {
        IfElse {
            if_clause,
            else_clause,
        }
    }
    pub fn execute(&'a self, environment: &'a Environment<'a>) -> Result<Option<FlowStatement>> {
        let mut flow_statement: Option<FlowStatement> = None;
        for (expr, scope) in self.if_clause.iter() {
            let expr_val = expr
                .execute(environment)
                .context("Failed to evaluate expression")?;
            if expr_val.as_bool()? {
                flow_statement = scope.execute(environment, false)?;
                return Ok(flow_statement);
            }
        }
        if let Some(scope) = &self.else_clause {
            flow_statement = scope.execute(environment, false)?;
        }
        Ok(flow_statement)
    }
}
