use std::{borrow::Borrow, str::FromStr};

use crate::parser::ast::ASTNode;
use anyhow::Result;

use super::{chess_notation::ChessNotation, primitive::Primitive};

#[derive(Debug)]
pub enum OpType {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug)]
pub struct Operation {
    pub op_type: OpType,
    pub chess_notation: ChessNotation,
}
#[derive(Debug)]
pub enum Expression {
    Literal {
        lhs: Box<dyn Primitive>,
    },
    Unary {
        lhs: Box<Expression>,
        op: Operation,
    },
    Binary {
        lhs: Box<Expression>,
        op: Operation,
        rhs: Box<Expression>,
    },
}

impl Expression {
    pub fn evaluation(&self) -> Result<Box<dyn Primitive>> {
        match self {
            Expression::Literal { lhs } => Ok(lhs.clone_box()),
            Expression::Unary { lhs, op } => unimplemented!(),
            Expression::Binary { lhs, op, rhs } => {
                let mut lhs_val: Box<dyn Primitive> = lhs.evaluation()?;
                let mut rhs_val: Box<dyn Primitive> = rhs.evaluation()?;
                Ok(lhs_val.evaluate(&rhs_val, op)?)
            }
        }
    }
}
