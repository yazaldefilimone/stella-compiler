// Copyright 2024 Yazalde Filimone <yazalde.filimone@gmail.com>
// Lua Virtual Machine Values
#![allow(dead_code)]

use crate::vm::ExeState;

#[derive(Debug, Clone)]
pub enum Value {
  Nil,
  Boolean(bool),
  Integer(i64),
  Float(f64),
  String(String),
  Function(fn(&mut ExeState) -> i32),
  // Table(Table),
}
