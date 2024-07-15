#![allow(dead_code)]

use std::cmp::Ordering;

use crate::values::Value;

pub struct Stack {
  stack: Vec<Value>,
}

impl Stack {
  pub fn new() -> Self {
    Stack { stack: vec![] }
  }

  pub fn push(&mut self, value: Value) {
    self.stack.push(value);
  }

  pub fn pop(&mut self) -> Value {
    self.stack.pop().unwrap()
  }

  pub fn peek(&self) -> Value {
    self.stack.last().unwrap().clone()
  }

  pub fn set(&mut self, destination: u8, value: Value) {
    let destination = destination as usize;
    match destination.cmp(&self.stack.len()) {
      Ordering::Equal => self.stack.push(value),
      Ordering::Less => self.stack[destination] = value,
      Ordering::Greater => panic!("fail in set_stack"),
    }
  }

  pub fn get(&mut self, destination: u8) -> Value {
    let destination = destination as usize;
    match destination.cmp(&self.stack.len()) {
      Ordering::Equal => self.stack.pop().unwrap().clone(),
      Ordering::Less => self.stack[destination].clone(),
      Ordering::Greater => panic!("fail in get_stack"),
    }
  }
  pub fn len(&self) -> usize {
    self.stack.len()
  }
}
