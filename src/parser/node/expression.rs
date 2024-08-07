use crate::environment::environment::{Environment, EnvironmentItem};
use anyhow::Result;

use super::primitive::Primitive;

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
    pub fn evaluation(&self, environment: &Environment) -> Result<Primitive> {
        match self {
            Expression::Literal { lhs } => match lhs {
                ExprAtom::Primitive(val) => Ok(*val),
                ExprAtom::Identifier(val) => match environment.get_var(val)? {
                    EnvironmentItem::Primitive(val) => Ok(*val),
                    EnvironmentItem::Func(val) => todo!(),
                },
            },
            Expression::Unary { lhs, op } => {
                let lhs_val = lhs.evaluation(environment)?;
                Ok(lhs_val.evaluate_unary(op)?)
            }
            Expression::Binary { lhs, op, rhs } => match op {
                OpType::And => {
                    let lhs_val = lhs.evaluation(environment)?;
                    if lhs_val.as_bool()? == false {
                        return Ok(lhs_val);
                    }
                    let rhs_val = rhs.evaluation(environment)?;
                    Ok(lhs_val.evaluate_primary(&rhs_val, op)?)
                }
                OpType::Or => {
                    let lhs_val = lhs.evaluation(environment)?;
                    if lhs_val.as_bool()? == true {
                        return Ok(lhs_val);
                    }
                    let rhs_val = rhs.evaluation(environment)?;
                    Ok(lhs_val.evaluate_primary(&rhs_val, op)?)
                }
                _ => {
                    let lhs_val = lhs.evaluation(environment)?;
                    let rhs_val = rhs.evaluation(environment)?;
                    Ok(lhs_val.evaluate_primary(&rhs_val, op)?)
                }
            },
        }
    }
}
