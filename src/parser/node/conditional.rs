use crate::environment::environment::Environment;

use super::{
    expression::Expression,
    scope::{FlowStatement, Scope},
};
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
    pub fn execute(
        &'a self,
        mut environment: Environment<'a>,
    ) -> Result<(Environment, Option<FlowStatement>)> {
        let mut flow_statement: Option<FlowStatement> = None;
        for (expr, scope) in self.if_clause.iter() {
            let (env, expr_val) = expr
                .execute(environment)
                .context("Failed to evaluate expression")?;
            if expr_val.as_bool()? {
                (environment, flow_statement) = scope.execute(env)?;
                return Ok((environment, flow_statement));
            }
            environment = env;
        }
        if let Some(scope) = &self.else_clause {
            (environment, flow_statement) = scope.execute(environment)?;
        }
        Ok((environment, flow_statement))
    }
}
