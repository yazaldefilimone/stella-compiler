use std::fmt::Display;

use crate::values::Value;

impl Display for Value {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Value::Nil => write!(f, "nil"),
      Value::Boolean(b) => write!(f, "{}", b),
      Value::Number(n) => write!(f, "{}", n),
      Value::String(s) => write!(f, "{}", s),
      Value::Function(_) => write!(f, "function"),
    }
  }
}
