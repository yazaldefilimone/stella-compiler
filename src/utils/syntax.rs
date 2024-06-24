use crate::ast::token::{Token, TokenKind};

pub const SYNTAX_KEYWORDS: [&str; 6] = ["function", "end", "if", "then", "local", "return"];
pub const SYNTAX_OPERATORS: [&str; 12] = ["+", "-", "*", "/", "^", "%", "==", "~=", "<", ">", "<=", ">="];
pub const SYNTAX_LITERALS: [&str; 3] = ["true", "false", "nil"];

pub fn expect_keyword(tokens: &[Token], index: usize, value: &str) -> bool {
  if index >= tokens.len() {
    return false;
  }

  let t = tokens[index].clone();
  t.kind == TokenKind::Keyword && t.value == value
}

pub fn expect_syntax(tokens: &[Token], index: usize, value: &str) -> bool {
  if index >= tokens.len() {
    return false;
  }

  let t = tokens[index].clone();
  t.kind == TokenKind::Syntax && t.value == value
}

pub fn expect_identifier(tokens: &[Token], index: usize) -> bool {
  if index >= tokens.len() {
    return false;
  }

  let t = tokens[index].clone();
  t.kind == TokenKind::Identifier
}
