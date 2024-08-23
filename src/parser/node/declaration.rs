use anyhow::Ok;
use anyhow::Result;

use crate::environment::environment::Var;
use crate::ProgramState;

use super::expression::Expr;
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
pub struct Assignment {
    var: Var,
    op: AssignOperation,
    expr: Expr,
}

impl Assignment {
    pub fn new(var: Var, op: AssignOperation, expr: Expr) -> Self {
        Self { var, op, expr }
    }
    pub fn execute(&self, program: &ProgramState) -> Result<()> {
        let val = program.environment.borrow().get_var(self.var)?;
        let expr_val = match self.op {
            AssignOperation::AssignAdd => {
                let v = self.expr.execute(program)?;
                v.evaluate_primary(&val, &OpType::Add)
            }
            AssignOperation::AssignSub => {
                let v = self.expr.execute(program)?;
                v.evaluate_primary(&val, &OpType::Sub)
            }
            AssignOperation::AssignMul => {
                let v = self.expr.execute(program)?;
                v.evaluate_primary(&val, &OpType::Mul)
            }
            AssignOperation::AssignDiv => {
                let v = self.expr.execute(program)?;
                v.evaluate_primary(&val, &OpType::Div)
            }
            AssignOperation::AssignOp => {
                let v = self.expr.execute(program)?;
                Ok(v)
            }
        }?;
        program
            .environment
            .borrow_mut()
            .assign_var(self.var, expr_val)
    }
}

#[derive(Debug)]
pub enum Declaration {
    Expression { var: Var, expr: Expr },
    Function { var: Var, func: Rc<Func> },
}
impl Declaration {
    pub fn execute(&self, program: &ProgramState) -> Result<()> {
        match self {
            Self::Expression { var, expr } => {
                let expr_val = expr.execute(program)?;
                program
                    .environment
                    .borrow_mut()
                    .subscribe_var(*var, expr_val)
            }
            Self::Function { var, func } => program
                .environment
                .borrow_mut()
                .subscribe_func(*var, Rc::clone(func)),
        }
    }
}
