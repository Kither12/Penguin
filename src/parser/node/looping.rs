use anyhow::Result;

use crate::environment::environment::Environment;

use super::{
    expression::ExpressionPool,
    scope::{FlowStatement, Scope},
};

#[derive(Debug)]
pub struct WhileLoop<'a> {
    expr_pool: ExpressionPool<'a>,
    scope: Scope<'a>,
}

impl<'a> WhileLoop<'a> {
    pub fn new(expr_pool: ExpressionPool<'a>, scope: Scope<'a>) -> Self {
        WhileLoop { expr_pool, scope }
    }
    pub fn execute(&'a self, environment: &'a Environment<'a>) -> Result<Option<FlowStatement>> {
        let mut expr_val = self.expr_pool.execute(environment)?;
        let mut flow_statement: Option<FlowStatement> = None;
        while expr_val.as_bool()? {
            let v = self.scope.execute(environment, false)?;
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
            expr_val = self.expr_pool.execute(environment)?;
        }
        Ok(flow_statement)
    }
}
