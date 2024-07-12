use anyhow::{anyhow, Result};
use core::fmt::Debug;
use std::{
    any::{Any, TypeId},
    fmt::Formatter,
    str::FromStr,
};

use super::expression::{OpType, Operation};

pub trait Primitive {
    fn evaluate(&self, other: &Box<dyn Primitive>, op: &Operation) -> Result<Box<dyn Primitive>> {
        Err(anyhow!("Unvalid operation"))
    }
    fn clone_box(&self) -> Box<dyn Primitive>;
    fn as_int(&self) -> Result<Integer> {
        Err(anyhow!("Cannot cast to integer"))
    }
    fn debug(&self, f: &mut Formatter) -> core::fmt::Result;
}

impl Debug for dyn Primitive {
    fn fmt(&self, f: &mut Formatter) -> core::fmt::Result {
        self.debug(f)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Integer {
    value: i128,
}

impl FromStr for Integer {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            value: s.parse::<i128>()?,
        })
    }
}

impl Primitive for Integer {
    fn debug(&self, f: &mut Formatter) -> core::fmt::Result {
        write!(f, "{}", self.value)
    }
    fn evaluate(&self, other: &Box<dyn Primitive>, op: &Operation) -> Result<Box<dyn Primitive>> {
        match op.op_type {
            OpType::Add => Ok(Box::new(Integer {
                value: self.value + other.as_int()?.value,
            })),
            OpType::Sub => Ok(Box::new(Integer {
                value: self.value - other.as_int()?.value,
            })),
            OpType::Mul => Ok(Box::new(Integer {
                value: self.value * other.as_int()?.value,
            })),
            OpType::Div => Ok(Box::new(Integer {
                value: self.value / other.as_int()?.value,
            })),
            _ => Err(anyhow!("Unvalid operation")),
        }
    }
    fn as_int(&self) -> Result<Integer> {
        Ok(*self)
    }

    fn clone_box(&self) -> Box<dyn Primitive> {
        Box::new(Self::clone(self))
    }
}
