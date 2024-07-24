use anyhow::Result;
use core::fmt::Debug;
use std::{fmt::Formatter, str::FromStr};

use super::expression::OpType;

pub trait Primitive {
    fn evaluate_primary(
        &self,
        other: &Box<dyn Primitive>,
        op: &OpType,
    ) -> Result<Box<dyn Primitive>> {
        match op {
            OpType::Add => Ok(Box::new(Integer {
                value: self.as_int() + other.as_int(),
            })),
            OpType::Sub => Ok(Box::new(Integer {
                value: self.as_int() - other.as_int(),
            })),
            OpType::Mul => Ok(Box::new(Integer {
                value: self.as_int() * other.as_int(),
            })),
            OpType::Div => Ok(Box::new(Integer {
                value: self.as_int() / other.as_int(),
            })),
            OpType::Mod => Ok(Box::new(Integer {
                value: self.as_int() % other.as_int(),
            })),
            OpType::And => Ok(Box::new(Boolean {
                value: self.as_bool() && other.as_bool(),
            })),
            OpType::Or => Ok(Box::new(Boolean {
                value: self.as_bool() || other.as_bool(),
            })),
            OpType::Gt => Ok(Box::new(Boolean {
                value: self.as_int() > other.as_int(),
            })),
            OpType::Lt => Ok(Box::new(Boolean {
                value: self.as_int() < other.as_int(),
            })),
            OpType::Gte => Ok(Box::new(Boolean {
                value: self.as_int() >= other.as_int(),
            })),
            OpType::Lte => Ok(Box::new(Boolean {
                value: self.as_int() <= other.as_int(),
            })),
            OpType::Eq => Ok(Box::new(Boolean {
                value: self.as_int() == other.as_int(),
            })),
            OpType::Neq => Ok(Box::new(Boolean {
                value: self.as_int() != other.as_int(),
            })),
            _ => unreachable!(),
        }
    }
    fn evaluate_unary(&self, op: &OpType) -> Result<Box<dyn Primitive>> {
        match op {
            OpType::Add => Ok(Box::new(Integer {
                value: self.as_int(),
            })),
            OpType::Sub => Ok(Box::new(Integer {
                value: -self.as_int(),
            })),
            OpType::Opp => Ok(Box::new(Boolean {
                value: !self.as_bool(),
            })),
            _ => unreachable!(),
        }
    }
    fn clone_box(&self) -> Box<dyn Primitive>;
    fn as_int(&self) -> i64;
    fn as_bool(&self) -> bool;
    fn debug(&self, f: &mut Formatter) -> core::fmt::Result;
}

impl Debug for dyn Primitive {
    fn fmt(&self, f: &mut Formatter) -> core::fmt::Result {
        self.debug(f)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Integer {
    value: i64,
}

impl FromStr for Integer {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            value: s.parse::<i64>()?,
        })
    }
}

impl Primitive for Integer {
    fn debug(&self, f: &mut Formatter) -> core::fmt::Result {
        write!(f, "{}", self.value)
    }
    fn as_int(&self) -> i64 {
        self.value
    }
    fn as_bool(&self) -> bool {
        self.value != 0
    }

    fn clone_box(&self) -> Box<dyn Primitive> {
        Box::new(Self::clone(self))
    }
}
#[derive(Debug, Clone, Copy)]
pub struct Boolean {
    value: bool,
}
impl FromStr for Boolean {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "true" => Ok(Self { value: true }),
            "false" => Ok(Self { value: false }),
            _ => unreachable!(),
        }
    }
}

impl Primitive for Boolean {
    fn debug(&self, f: &mut Formatter) -> core::fmt::Result {
        write!(f, "{}", self.value)
    }
    fn as_int(&self) -> i64 {
        match self.value {
            true => 1,
            false => 0,
        }
    }
    fn as_bool(&self) -> bool {
        self.value
    }

    fn clone_box(&self) -> Box<dyn Primitive> {
        Box::new(Self::clone(self))
    }
}
