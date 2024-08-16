use anyhow::Result;

use crate::environment::environment::Environment;

use super::{
    expression::Expression,
    scope::{FlowStatement, Scope},
};

#[derive(Debug)]
pub struct WhileLoop<'a> {
    expr: Expression<'a>,
    scope: Scope<'a>,
}

impl<'a> WhileLoop<'a> {
    pub fn new(expr: Expression<'a>, scope: Scope<'a>) -> Self {
        WhileLoop { expr, scope }
    }
    pub fn execute(&'a self, environment: &Environment<'a>) -> Result<Option<FlowStatement>> {
        let mut expr_val = self.expr.execute(environment)?;
        let mut flow_statement: Option<FlowStatement> = None;
        while expr_val.as_bool()? {
            let v = self.scope.execute(environment)?;
            if let Some(flow) = v {
                match flow {
                    FlowStatement::Break => break,
                    FlowStatement::Return(v) => {
                        flow_statement = Some(FlowStatement::Return(v));
                        break;
                    }
                    FlowStatement::Continue => {}
                }
            }
            expr_val = self.expr.execute(environment)?;
        }
        Ok(flow_statement)
    }
}
