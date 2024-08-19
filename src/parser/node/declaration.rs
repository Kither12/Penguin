use anyhow::Ok;
use anyhow::Result;

use crate::environment::environment::Environment;

use super::expression::ExpressionPool;
use super::expression::OpType;
use super::function::Func;
use std::rc::Rc;

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
    expr_pool: ExpressionPool<'a>,
}

impl<'a> Assignment<'a> {
    pub fn new(identifier: &'a str, op: AssignOperation, expr_pool: ExpressionPool<'a>) -> Self {
        Self {
            identifier,
            op,
            expr_pool,
        }
    }
    pub fn execute(&'a self, environment: &'a Environment<'a>) -> Result<()> {
        let val = environment.get_var(&self.identifier)?;
        let expr_val = match self.op {
            AssignOperation::AssignAdd => {
                let v = self.expr_pool.execute(environment)?;
                v.evaluate_primary(&val, &OpType::Add)
            }
            AssignOperation::AssignSub => {
                let v = self.expr_pool.execute(environment)?;
                v.evaluate_primary(&val, &OpType::Sub)
            }
            AssignOperation::AssignMul => {
                let v = self.expr_pool.execute(environment)?;
                v.evaluate_primary(&val, &OpType::Mul)
            }
            AssignOperation::AssignDiv => {
                let v = self.expr_pool.execute(environment)?;
                v.evaluate_primary(&val, &OpType::Div)
            }
            AssignOperation::AssignOp => {
                let v = self.expr_pool.execute(environment)?;
                Ok(v)
            }
        }?;
        environment.assign_var(self.identifier, Rc::new(expr_val))
    }
}

#[derive(Debug)]
pub enum Declaration<'a> {
    Expression {
        identifier: &'a str,
        expr_pool: ExpressionPool<'a>,
    },
    Function {
        identifier: &'a str,
        func: Rc<Func<'a>>,
    },
}
impl<'a> Declaration<'a> {
    pub fn execute(&'a self, environment: &'a Environment<'a>) -> Result<()> {
        match self {
            Self::Expression {
                identifier,
                expr_pool,
            } => {
                let expr_val = expr_pool.execute(environment)?;
                environment.subscribe_var(identifier, Rc::new(expr_val))
            }
            Self::Function { identifier, func } => {
                environment.subscribe_func(identifier, Rc::clone(func))
            }
        }
    }
}
