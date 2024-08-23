use anyhow::Result;

use crate::ProgramState;

use super::{
    expression::Expr,
    scope::{FlowStatement, Scope},
};

#[derive(Debug)]
pub struct WhileLoop {
    expr: Expr,
    scope: Scope,
}

impl WhileLoop {
    pub fn new(expr: Expr, scope: Scope) -> Self {
        WhileLoop { expr, scope }
    }
    pub fn execute(&self, program: &ProgramState) -> Result<Option<FlowStatement>> {
        let mut expr_val = self.expr.execute(program)?;
        let mut flow_statement: Option<FlowStatement> = None;
        while expr_val.as_bool()? {
            let v = self.scope.execute(program, false)?;
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
            expr_val = self.expr.execute(program)?;
        }
        Ok(flow_statement)
    }
}
