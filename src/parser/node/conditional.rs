use crate::ProgramState;

use super::{
    expression::Expr,
    scope::{FlowStatement, Scope},
};
use anyhow::Result;

#[derive(Debug)]
pub struct IfElse {
    if_clause: Box<[(Expr, Scope)]>,
    else_clause: Option<Scope>,
}

impl IfElse {
    pub fn new(if_clause: Box<[(Expr, Scope)]>, else_clause: Option<Scope>) -> Self {
        IfElse {
            if_clause,
            else_clause,
        }
    }
    pub fn execute(&self, program: &ProgramState) -> Result<Option<FlowStatement>> {
        let mut flow_statement: Option<FlowStatement> = None;
        for (expr, scope) in self.if_clause.iter() {
            let expr_val = expr.execute(program)?;
            if expr_val.as_bool()? {
                flow_statement = scope.execute(program, false)?;
                return Ok(flow_statement);
            }
        }
        if let Some(scope) = &self.else_clause {
            flow_statement = scope.execute(program, false)?;
        }
        Ok(flow_statement)
    }
}
