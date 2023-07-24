use crate::error::RuntimeError;
use std::fmt::{write, Debug, Display};

#[repr(C)]
#[derive(Clone, Copy, PartialEq)]
pub enum Value {
    Nil,
    Boolean(bool),
    Number(f64),
}

impl From<f64> for Value {
    fn from(value: f64) -> Self {
        Self::Number(value)
    }
}

impl From<bool> for Value {
    fn from(value: bool) -> Self {
        Self::Boolean(value)
    }
}

impl Into<bool> for Value {
    fn into(self) -> bool {
        match self {
            Value::Nil | Value::Boolean(false) => false,
            _ => true,
        }
    }
}

impl Value {
    pub fn negate(&self) -> Result<Self, RuntimeError> {
        match self {
            Value::Number(n) => Ok(Value::Number(-n)),
            _ => Err(RuntimeError::OperandMustBeNumber),
        }
    }

    pub fn add(&self, other: Self) -> Result<Self, RuntimeError> {
        match (self, other) {
            (Value::Number(n1), Value::Number(n2)) => Ok(Value::Number(n1 + n2)),
            _ => Err(RuntimeError::OperandsMustBeNumber),
        }
    }

    pub fn subtract(&self, other: Self) -> Result<Self, RuntimeError> {
        match (self, other) {
            (Value::Number(n1), Value::Number(n2)) => Ok(Value::Number(n1 - n2)),
            _ => Err(RuntimeError::OperandsMustBeNumber),
        }
    }

    pub fn multiply(&self, other: Self) -> Result<Self, RuntimeError> {
        match (self, other) {
            (Value::Number(n1), Value::Number(n2)) => Ok(Value::Number(n1 * n2)),
            _ => Err(RuntimeError::OperandsMustBeNumber),
        }
    }

    pub fn divide(&self, other: Self) -> Result<Self, RuntimeError> {
        match (self, other) {
            (Value::Number(n1), Value::Number(n2)) => Ok(Value::Number(n1 / n2)),
            _ => Err(RuntimeError::OperandsMustBeNumber),
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Nil => write!(f, "nil"),
            Value::Boolean(b) => write!(f, "{}", b),
            Value::Number(n) => write!(f, "{}", n),
        }
    }
}

impl Debug for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}
