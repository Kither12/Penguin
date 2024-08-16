use anyhow::Ok;
use anyhow::Result;

use crate::environment::environment::Environment;
use crate::environment::environment::EnvironmentItem;

use super::expression::Expression;

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
    pub fn execute(&'a self, environment: &Environment<'a>) -> Result<()> {
        let var = environment.get_var(&self.identifier)?;
        let val = match var {
            EnvironmentItem::Primitive(val) => val,
            EnvironmentItem::Func(val) => todo!(),
        };
        let expr_val = match self.op {
            AssignOperation::AssignAdd => {
                let v = self.expr.execute(environment)?;
                v.evaluate_primary(&val, &OpType::Add)
            }
            AssignOperation::AssignSub => {
                let v = self.expr.execute(environment)?;
                v.evaluate_primary(&val, &OpType::Sub)
            }
            AssignOperation::AssignMul => {
                let v = self.expr.execute(environment)?;
                v.evaluate_primary(&val, &OpType::Mul)
            }
            AssignOperation::AssignDiv => {
                let v = self.expr.execute(environment)?;
                v.evaluate_primary(&val, &OpType::Div)
            }
            AssignOperation::AssignOp => {
                let v = self.expr.execute(environment)?;
                Ok(v)
            }
        }?;
        environment.assign_var(self.identifier, EnvironmentItem::Primitive(expr_val))
    }
}

#[derive(Debug)]
pub enum Declaration<'a> {
    Expression {
        identifier: &'a str,
        expr: Expression<'a>,
    },
    Function {
        identifier: &'a str,
        func: Rc<Func<'a>>,
    },
}
impl<'a> Declaration<'a> {
    pub fn execute(&'a self, environment: &Environment<'a>) -> Result<()> {
        match self {
            Self::Expression { identifier, expr } => {
                let expr_val = expr.execute(environment)?;
                environment.subscribe(identifier, EnvironmentItem::Primitive(expr_val))
            }
            Self::Function { identifier, func } => {
                environment.subscribe(identifier, EnvironmentItem::Func(Rc::clone(func)))
            }
        }
    }
}
