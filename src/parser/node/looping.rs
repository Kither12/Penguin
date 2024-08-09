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
    pub fn execute(
        &'a self,
        environment: Environment<'a>,
    ) -> Result<(Environment<'a>, Option<FlowStatement>)> {
        let (mut env, mut expr_val) = self.expr.execute(environment)?;
        let mut flow_statement: Option<FlowStatement> = None;
        while expr_val.as_bool()? {
            let v = self.scope.execute(env)?;
            env = v.0;
            if let Some(flow) = v.1 {
                match flow {
                    FlowStatement::Break => break,
                    FlowStatement::Return => {
                        flow_statement = Some(FlowStatement::Return);
                        break;
                    }
                    FlowStatement::Continue => {}
                }
            }
            (env, expr_val) = self.expr.execute(env)?;
        }
        Ok((env, flow_statement))
    }
}
