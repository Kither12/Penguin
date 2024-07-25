use anyhow::Result;
use core::fmt::Debug;

use super::expression::OpType;

#[derive(Debug, Clone, Copy)]
pub enum Primitive {
    Integer(i64),
    Boolean(bool),
}
impl Primitive {
    pub fn as_int(&self) -> Result<i64> {
        match self {
            Primitive::Boolean(v) => Ok(*v as i64),
            Primitive::Integer(v) => Ok(*v),
        }
    }
    pub fn as_bool(&self) -> Result<bool> {
        match self {
            Primitive::Boolean(v) => Ok(*v),
            Primitive::Integer(v) => Ok(*v > 0),
        }
    }
    pub fn evaluate_primary(&self, other: &Primitive, op: &OpType) -> Result<Primitive> {
        match op {
            OpType::Add => Ok(Primitive::Integer(self.as_int()? + other.as_int()?)),
            OpType::Sub => Ok(Primitive::Integer(self.as_int()? - other.as_int()?)),
            OpType::Mul => Ok(Primitive::Integer(self.as_int()? * other.as_int()?)),
            OpType::Div => Ok(Primitive::Integer(self.as_int()? / other.as_int()?)),
            OpType::Mod => Ok(Primitive::Integer(self.as_int()? % other.as_int()?)),
            OpType::And => Ok(Primitive::Boolean(self.as_bool()? && other.as_bool()?)),
            OpType::Or => Ok(Primitive::Boolean(self.as_bool()? || other.as_bool()?)),
            OpType::Gt => Ok(Primitive::Boolean(self.as_int()? > other.as_int()?)),
            OpType::Lt => Ok(Primitive::Boolean(self.as_int()? < other.as_int()?)),
            OpType::Gte => Ok(Primitive::Boolean(self.as_int()? >= other.as_int()?)),
            OpType::Lte => Ok(Primitive::Boolean(self.as_int()? <= other.as_int()?)),
            OpType::Eq => Ok(Primitive::Boolean(self.as_int()? == other.as_int()?)),
            OpType::Neq => Ok(Primitive::Boolean(self.as_int()? != other.as_int()?)),
            OpType::BitAnd => Ok(Primitive::Integer(self.as_int()? & other.as_int()?)),
            OpType::BitOr => Ok(Primitive::Integer(self.as_int()? | other.as_int()?)),
            OpType::BitXor => Ok(Primitive::Integer(self.as_int()? ^ other.as_int()?)),
            OpType::ShiftLeft => Ok(Primitive::Integer(self.as_int()? << other.as_int()?)),
            OpType::ShiftRight => Ok(Primitive::Integer(self.as_int()? >> other.as_int()?)),
            _ => unreachable!(),
        }
    }
    pub fn evaluate_unary(&self, op: &OpType) -> Result<Primitive> {
        match op {
            OpType::Add => Ok(Primitive::Integer(self.as_int()?)),
            OpType::Sub => Ok(Primitive::Integer(-self.as_int()?)),
            OpType::Opp => Ok(Primitive::Boolean(!self.as_bool()?)),
            OpType::BitNot => Ok(Primitive::Integer(!self.as_int()?)),
            _ => unreachable!(),
        }
    }
}
