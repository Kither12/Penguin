use std::cell::RefCell;
use std::rc::Rc;

use crate::environment::environment::Environment;
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
pub enum ExprAtom<'a> {
    Primitive(Primitive),
    FunctionCall(FunctionCall<'a>),
    Identifier(&'a str),
}
#[derive(Debug)]
pub struct ExpressionPool<'a> {
    pool: Vec<Expression<'a>>,
}

impl<'a> ExpressionPool<'a> {
    pub fn new() -> Self {
        ExpressionPool {
            pool: Vec::with_capacity(64),
        }
    }
    pub fn add(&mut self, expr: Expression<'a>) -> usize {
        self.pool.push(expr);
        self.pool.len() - 1
    }
    pub fn get(&'a self, idx: usize) -> &Expression {
        self.pool.get(idx).unwrap()
    }
    pub fn execute(&'a self, environment: &'a Environment<'a>) -> Result<Primitive> {
        self.pool.last().unwrap().execute(environment, self)
    }
}

#[derive(Debug)]
pub enum Expression<'a> {
    Literal { lhs: ExprAtom<'a> },
    Unary { lhs: usize, op: OpType },
    Binary { lhs: usize, op: OpType, rhs: usize },
}

impl<'a> Expression<'a> {
    pub fn execute(
        &'a self,
        environment: &'a Environment<'a>,
        expr_pool: &'a ExpressionPool<'a>,
    ) -> Result<Primitive> {
        match self {
            Expression::Literal { lhs } => match lhs {
                ExprAtom::Primitive(val) => Ok(*val),

                ExprAtom::FunctionCall(val) => val.execute(environment),
                ExprAtom::Identifier(val) => environment.get_var(val).map(|v| *v),
            },
            Expression::Unary { lhs, op } => {
                let lhs_val = expr_pool.get(*lhs).execute(environment, expr_pool)?;
                Ok(lhs_val.evaluate_unary(op)?)
            }
            Expression::Binary { lhs, op, rhs } => match op {
                OpType::And => {
                    let lhs_val = expr_pool.get(*lhs).execute(environment, expr_pool)?;
                    if lhs_val.as_bool()? == false {
                        return Ok(lhs_val);
                    }
                    let rhs_val = expr_pool.get(*rhs).execute(environment, expr_pool)?;
                    Ok(lhs_val.evaluate_primary(&rhs_val, op)?)
                }
                OpType::Or => {
                    let lhs_val = expr_pool.get(*lhs).execute(environment, expr_pool)?;
                    if lhs_val.as_bool()? == true {
                        return Ok(lhs_val);
                    }
                    let rhs_val = expr_pool.get(*rhs).execute(environment, expr_pool)?;
                    Ok(lhs_val.evaluate_primary(&rhs_val, op)?)
                }
                _ => {
                    let lhs_val = expr_pool.get(*lhs).execute(environment, expr_pool)?;
                    let rhs_val = expr_pool.get(*rhs).execute(environment, expr_pool)?;
                    Ok(lhs_val.evaluate_primary(&rhs_val, op)?)
                }
            },
        }
    }
}
