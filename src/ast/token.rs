#![allow(dead_code)]
use crate::vm::ExeState;

#[derive(Debug)]
pub enum Token {
  Name(String),
  String(String),
  Function(fn(&mut ExeState) -> i32),
  Eof,
}
