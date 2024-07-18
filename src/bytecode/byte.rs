#![allow(dead_code)]
use crate::values::Value;

use super::ByteCode;

pub fn add_constant(constants: &mut Vec<Value>, value: Value) -> u8 {
  if let Some(index) = constants.iter().position(|constant| constant == &value) {
    return index as u8;
  }
  constants.push(value);
  let last_index = constants.len() - 1;
  return last_index as u8;
}

pub fn load_constant(constants: &mut Vec<Value>, destination: usize, value: Value) -> ByteCode {
  let constant_index = add_constant(constants, value) as u8;
  return ByteCode::LoadConst(destination as u8, constant_index);
}

pub fn get_global(destination: u8, constant_index: u8) -> ByteCode {
  return ByteCode::GetGlobal(destination, constant_index);
}

pub fn call(calleble_ref: u8, index: u8) -> ByteCode {
  return ByteCode::Call(calleble_ref, index);
}

pub fn load_boolean(destination: u8, boolean: bool) -> ByteCode {
  return ByteCode::LoadBoolean(destination, boolean);
}

pub fn load_integer(destination: u8, integer: i64) -> ByteCode {
  return ByteCode::LoadInteger(destination, integer);
}

pub fn load_float(destination: u8, float: f64) -> ByteCode {
  return ByteCode::LoadFloat(destination, float);
}

pub fn load_nil(destination: u8) -> ByteCode {
  return ByteCode::LoadNil(destination);
}

pub fn nop() -> ByteCode {
  return ByteCode::Nop;
}
