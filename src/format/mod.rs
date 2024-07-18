use std::fmt::Debug;

use crate::{bytecode::ByteCode, values::Value};

impl Debug for Value {
  fn fmt(&self, format: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Value::Nil => write!(format, "nil"),
      Value::Boolean(boolean) => write!(format, "{}", boolean),
      Value::String(string) => write!(format, "{}", string),
      Value::Function(_) => write!(format, "function"),
      Value::Integer(integer) => write!(format, "{}", integer),
      Value::Float(float) => write!(format, "{:?}", float),
    }
  }
}

impl Debug for ByteCode {
  fn fmt(&self, format: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      ByteCode::LoadConst(destination, constant_index) => {
        write!(format, "LoadConst({}, {})", destination, constant_index)
      }
      ByteCode::GetGlobal(destination, constant_index) => {
        write!(format, "GetGlobal({}, {})", destination, constant_index)
      }
      ByteCode::Call(calleble_ref, index) => write!(format, "Call({}, {})", calleble_ref, index),
      ByteCode::LoadBoolean(destination, boolean) => write!(format, "LoadBoolean({}, {})", destination, boolean),
      ByteCode::LoadInteger(destination, integer) => write!(format, "LoadInteger({}, {})", destination, integer),
      ByteCode::LoadFloat(destination, float) => write!(format, "LoadFloat({}, {})", destination, float),
      ByteCode::LoadNil(destination) => write!(format, "LoadNil({})", destination),
      ByteCode::Nop => write!(format, "Nop"),
    }
  }
}
