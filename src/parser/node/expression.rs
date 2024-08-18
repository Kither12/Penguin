use crate::environment::environment::Environment;
use anyhow::{anyhow, Ok, Result};

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
pub enum Expression<'a> {
    Literal {
        lhs: ExprAtom<'a>,
    },
    Unary {
        lhs: Box<Expression<'a>>,
        op: OpType,
    },
    Binary {
        lhs: Box<Expression<'a>>,
        op: OpType,
        rhs: Box<Expression<'a>>,
    },
}

impl<'a> Expression<'a> {
    pub fn execute(&'a self, environment: &'a Environment<'a>) -> Result<Primitive> {
        match self {
            Expression::Literal { lhs } => match lhs {
                ExprAtom::Primitive(val) => Ok(*val),

                ExprAtom::FunctionCall(val) => val.execute(environment),
                ExprAtom::Identifier(val) => environment.get_var(val).map(|v| *v),
            },
            Expression::Unary { lhs, op } => {
                let lhs_val = lhs.execute(environment)?;
                Ok(lhs_val.evaluate_unary(op)?)
            }
            Expression::Binary { lhs, op, rhs } => match op {
                OpType::And => {
                    let lhs_val = lhs.execute(environment)?;
                    if lhs_val.as_bool()? == false {
                        return Ok(lhs_val);
                    }
                    let rhs_val = rhs.execute(environment)?;
                    Ok(lhs_val.evaluate_primary(&rhs_val, op)?)
                }
                OpType::Or => {
                    let lhs_val = lhs.execute(environment)?;
                    if lhs_val.as_bool()? == true {
                        return Ok(lhs_val);
                    }
                    let rhs_val = rhs.execute(environment)?;
                    Ok(lhs_val.evaluate_primary(&rhs_val, op)?)
                }
                _ => {
                    let lhs_val = lhs.execute(environment)?;
                    let rhs_val = rhs.execute(environment)?;
                    Ok(lhs_val.evaluate_primary(&rhs_val, op)?)
                }
            },
        }
    }
}
