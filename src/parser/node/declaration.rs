use anyhow::Result;

use crate::environment::environment::Environment;

use super::expression::Expression;

use super::expression::OpType;

#[derive(Debug)]
pub enum AssignOperation {
    AssignOp,
    AssignAdd,
    AssignSub,
    AssignMul,
    AssignDiv,
}

#[derive(Debug)]
pub struct Assignment<'a> {
    identifier: &'a str,
    op: AssignOperation,
    expr: Expression<'a>,
}

impl<'a> Assignment<'a> {
    pub fn new(identifier: &'a str, op: AssignOperation, expr: Expression<'a>) -> Self {
        Self {
            identifier: identifier,
            op: op,
            expr: expr,
        }
    }
    pub fn execute(&self, mut environment: Environment<'a>) -> Result<Environment> {
        let val = environment.get_var(&self.identifier)?;
        let expr_val = match self.op {
            AssignOperation::AssignAdd => self
                .expr
                .evaluation(&environment)
                .and_then(|v| v.evaluate_primary(val, &OpType::Add)),
            AssignOperation::AssignSub => self
                .expr
                .evaluation(&environment)
                .and_then(|v| v.evaluate_primary(val, &OpType::Sub)),
            AssignOperation::AssignMul => self
                .expr
                .evaluation(&environment)
                .and_then(|v| v.evaluate_primary(val, &OpType::Mul)),
            AssignOperation::AssignDiv => self
                .expr
                .evaluation(&environment)
                .and_then(|v| v.evaluate_primary(val, &OpType::Div)),
            AssignOperation::AssignOp => self.expr.evaluation(&environment),
        }?;

        environment = environment.assign_var(self.identifier, expr_val)?;
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
