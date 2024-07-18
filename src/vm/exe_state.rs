use std::cmp::Ordering;
use std::collections::HashMap;

use crate::bytecode::ByteCode;
use crate::compiler::Book;
use crate::stdlib::base::lib_print;
use crate::{stack::Stack, values::Value};

pub struct ExeState {
  pub stack: Stack,
  pub globals: HashMap<String, Value>,
}

impl ExeState {
  pub fn new() -> Self {
    let mut globals = HashMap::new();
    globals.insert("print".to_string(), Value::Function(lib_print));
    ExeState { stack: Stack::new(), globals }
  }

  pub fn get_global(&self, name: &str) -> Option<&Value> {
    self.globals.get(name)
  }

  pub fn set_global(&mut self, name: &str, value: Value) {
    self.globals.insert(name.to_string(), value);
  }

  pub fn execute(&mut self, book: &Book) {
    for byte_code in book.byte_codes.iter() {
      self.execute_byte_code(byte_code, book);
    }
  }

  pub fn execute_byte_code(&mut self, byte_code: &ByteCode, book: &Book) {
    match byte_code {
      ByteCode::GetGlobal(destination, name) => {
        let current_value_reference = book.constants[*name as usize].clone();
        if let Value::String(current_value) = current_value_reference {
          let value = self.get_global(&current_value).unwrap_or(&Value::Nil).clone();
          self.stack.set(*destination, value)
        } else {
          panic!("invalid global key: {:?}", name);
        }
      }
      ByteCode::LoadConstant(destination, constant_index) => {
        let value = book.constants[*constant_index as usize].clone();
        self.stack.set(*destination, value);
      }
      ByteCode::Call(calleble_ref, _index) => {
        let calleble_value = self.stack.get(*calleble_ref);
        if let Value::Function(function_call) = calleble_value {
          function_call(self);
        } else {
          panic!("invalid function: {:?}", calleble_value);
        }
      }
      _ => panic!("invalid byte code: {:?}", byte_code),
    };
  }
}
