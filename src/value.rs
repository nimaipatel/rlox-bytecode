use crate::{error::RuntimeError, object::ObjPtr};
use std::fmt::{write, Debug, Display};

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Value {
    Nil,
    Boolean(bool),
    Number(f64),
    ObjPtr(ObjPtr), // heap allocated object
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

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Nil => write!(f, "nil"),
            Value::Boolean(b) => write!(f, "{}", b),
            Value::Number(n) => write!(f, "{}", n),
            Value::ObjPtr(o) => write!(f, "{}", o),
        }
    }
}
