use std::fmt::Display;

use crate::values::Value;

impl Display for Value {
  fn fmt(&self, format: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Value::Nil => write!(format, "nil"),
      Value::Boolean(boolean) => write!(format, "{}", boolean),
      Value::String(string) => write!(format, "{}", string),
      Value::Function(_) => write!(format, "function"),
      Value::Integer(integer) => write!(format, "{}", integer),
      Value::Float(float) => write!(format, "{}", float),
    }
  }
}
