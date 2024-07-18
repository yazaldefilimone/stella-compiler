// Copyright 2024 Yazalde Filimone <yazalde.filimone@gmail.com>
// Lua Virtual Machine Values
#![allow(dead_code)]

use crate::vm::VirtualMachine;

#[derive(Clone)]
pub enum Value {
  Nil,
  Boolean(bool),
  Integer(i64),
  Float(f64),
  String(String),
  Function(fn(&mut VirtualMachine) -> i32),
  // Table(Table),
}

impl PartialEq for Value {
  fn eq(&self, other: &Self) -> bool {
    // TODO compare Integer vs Float
    match (self, other) {
      (Value::Nil, Value::Nil) => true,
      (Value::Boolean(b1), Value::Boolean(b2)) => *b1 == *b2,
      (Value::Integer(i1), Value::Integer(i2)) => *i1 == *i2,
      (Value::Float(f1), Value::Float(f2)) => *f1 == *f2,
      (Value::String(s1), Value::String(s2)) => *s1 == *s2,
      (Value::Function(f1), Value::Function(f2)) => std::ptr::eq(f1, f2),
      (_, _) => false,
    }
  }
}
