// Copyright 2024 Yazalde Filimone <yazalde.filimone@gmail.com>
//
// Virtual Machine Execution
//

use std::cmp::Ordering;
use std::collections::HashMap;

use crate::bytecode::ByteCode;
use crate::compiler::Book;
use crate::stdlib::base::lib_print;
use crate::{stack::Stack, values::Value};

pub struct VirtualMachine {
  pub stack: Stack,
  pub globals: HashMap<String, Value>,
}

impl VirtualMachine {
  pub fn new() -> Self {
    let mut globals = HashMap::new();
    globals.insert("print".to_string(), Value::Function(lib_print));
    VirtualMachine { stack: Stack::new(), globals }
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
        match current_value_reference {
          Value::String(current_value) => {
            let value = self.get_global(&current_value).unwrap_or(&Value::Nil).clone();
            self.stack.set(*destination, value)
          }
          _ => panic!("invalid global key: {:?}", name),
        }
      }

      ByteCode::Call(calleble_ref, _index) => {
        let calleble_value = self.stack.get(*calleble_ref);
        if let Value::Function(fn_call) = calleble_value {
          fn_call(self);
        } else {
          panic!("invalid function: {:?}", calleble_value);
        }
      }
      ByteCode::LoadConst(destination, constant_index) => {
        let value = book.constants[*constant_index as usize].clone();
        self.stack.set(*destination, value);
      }
      ByteCode::LoadBoolean(destination, boolean) => {
        self.stack.set(*destination, Value::Boolean(*boolean));
      }
      ByteCode::LoadInteger(destination, integer) => {
        self.stack.set(*destination, Value::Integer(*integer));
      }
      ByteCode::LoadFloat(destination, float) => {
        self.stack.set(*destination, Value::Float(*float));
      }
      ByteCode::LoadNil(destination) => {
        self.stack.set(*destination, Value::Nil);
      }
      _ => panic!("invalid byte code: {:?}", byte_code),
    };
  }
}
