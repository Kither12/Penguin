use crate::{environment::environment::Var, ProgramState};
use anyhow::Result;

use super::{function::FunctionCall, primitive::Primitive};

#[derive(Debug)]
pub enum OpType {
    Add,
    Sub,
    Mul,
    Div,
    And,
    Or,
    Opp,
    Gt,
    Lt,
    Gte,
    Lte,
    Eq,
    Neq,
    Mod,
    ShiftLeft,
    ShiftRight,
    BitAnd,
    BitOr,
    BitXor,
    BitNot,
}

#[derive(Debug)]
pub struct Expr(pub usize);

impl Expr {
    pub fn execute(&self, program: &ProgramState) -> Result<Primitive> {
        program.expr_pool.pool[self.0].execute(program)
    }
}

#[derive(Debug)]
pub enum ExprAtom {
    Primitive(Primitive),
    FunctionCall(FunctionCall),
    Var(Var),
}
#[derive(Debug)]
pub struct ExpressionPool {
    pool: Vec<Expression>,
}

impl Default for ExpressionPool {
    fn default() -> Self {
        Self::new()
    }
}

impl ExpressionPool {
    pub fn new() -> Self {
        ExpressionPool {
            pool: Vec::with_capacity(65536), // 2^16
        }
    }
    pub fn shrink(&mut self) {
        self.pool.shrink_to_fit();
    }
    pub fn add(&mut self, expr: Expression) -> usize {
        self.pool.push(expr);
        self.pool.len() - 1
    }
    pub fn execute(program_state: &ProgramState) -> Result<Primitive> {
        program_state
            .expr_pool
            .pool
            .last()
            .unwrap()
            .execute(program_state)
    }
}

#[derive(Debug)]
pub enum Expression {
    Literal { lhs: ExprAtom },
    Unary { lhs: Expr, op: OpType },
    Binary { lhs: Expr, op: OpType, rhs: Expr },
}

impl Expression {
    pub fn execute(&self, program: &ProgramState) -> Result<Primitive> {
        match self {
            Expression::Literal { lhs } => match lhs {
                ExprAtom::Primitive(val) => Ok(*val),

                ExprAtom::FunctionCall(val) => val.execute(program),
                ExprAtom::Var(val) => program.environment.borrow_mut().get_var(*val),
            },
            Expression::Unary { lhs, op } => {
                let lhs_val = program.expr_pool.pool[lhs.0].execute(program)?;
                Ok(lhs_val.evaluate_unary(op)?)
            }
            Expression::Binary { lhs, op, rhs } => match op {
                OpType::And => {
                    let lhs_val = program.expr_pool.pool[lhs.0].execute(program)?;
                    if !(lhs_val.as_bool()?) {
                        return Ok(lhs_val);
                    }
                    let rhs_val = program.expr_pool.pool[rhs.0].execute(program)?;
                    Ok(lhs_val.evaluate_primary(&rhs_val, op)?)
                }
                OpType::Or => {
                    let lhs_val = program.expr_pool.pool[lhs.0].execute(program)?;
                    if lhs_val.as_bool()? {
                        return Ok(lhs_val);
                    }
                    let rhs_val = program.expr_pool.pool[rhs.0].execute(program)?;
                    Ok(lhs_val.evaluate_primary(&rhs_val, op)?)
                }
                _ => {
                    let lhs_val = program.expr_pool.pool[lhs.0].execute(program)?;
                    let rhs_val = program.expr_pool.pool[rhs.0].execute(program)?;
                    Ok(lhs_val.evaluate_primary(&rhs_val, op)?)
                }
            },
        }
    }
}
