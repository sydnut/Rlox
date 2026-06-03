use std::cmp::Ordering;
use std::fmt::{Display, Formatter};

#[derive(Debug,Clone)]
pub enum Value{
    Double(f64),
    Boolean(bool),
    Nil
}
#[derive(Debug)]
#[derive(Clone)]
pub struct ValueArray {
    values: Vec<Value>,
}
impl ValueArray {
    pub fn new() -> ValueArray {
        ValueArray { values: Vec::new() }
    }
    pub fn write_value(&mut self, value: Value) {
        self.values.push(value);
    }
    pub fn count(&self) -> usize {
        self.values.len()
    }
    pub fn capacity(&self) -> usize {
        self.values.capacity()
    }
    pub fn values(&self) -> &[Value] {
        &self.values
    }
}
impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Double(v) => write!(f, "Double[{}]", v),
            Value::Boolean(v) => write!(f, "Bool[{}]", v),
            Value::Nil => write!(f, "Nil")
        }
    }
}
impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self,other) {
            (Value::Nil, Value::Nil) => true,
            (Value::Boolean(v1), Value::Boolean(v2)) => v1 == v2,
            (Value::Double(v1), Value::Double(v2)) => v1 == v2,
            _ => false
        }
    }
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self,other) {
            (Value::Double(v1), Value::Double(v2)) => v1.partial_cmp(v2),
            _ => None
        }
    }
}

impl Value{
    /// Value作为条件时是否为真
    /// ___
    /// only 0.0、nil、false为假
    pub fn is_truthy(&self) -> bool {
        match self {
            Value::Double(0f64) => false,
            Value::Double(_) => true,
            Value::Boolean(v) => *v,
            Value::Nil => false
        }
    }
}
