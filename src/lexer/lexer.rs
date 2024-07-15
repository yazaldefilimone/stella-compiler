#![allow(dead_code, unused_imports)]

use crate::ast::Token;

#[derive(Debug)]
pub struct Lexer {
  raw: String,
  cursor: usize,
}

impl Lexer {
  pub fn new(raw: String) -> Self {
    Lexer { raw, cursor: 0 }
  }

  pub fn next_token(&mut self) -> Token {
    self.skip_whitespace();
    if self.is_end() {
      return Token::Eof;
    }

    let current_character = self.peek_one();
    match current_character {
      'a'..='z' | 'A'..='Z' | '_' => self.lex_name(),
      '"' => self.lex_string(),
      // '0'..='9' => self.lex_number(),
      _ => panic!("Unexpected charcter '{}'", current_character),
    }
  }

  fn lex_string(&mut self) -> Token {
    self.consume_expect("\"");
    let text = self.consume_while(|character| character != '"');
    self.consume_expect("\"");
    Token::String(text)
  }

  // fn lex_number(&self) -> Token {
  //   let text = self.consume_while(|c| c.is_ascii_digit());
  //   Token::Number(text)
  // }
  pub fn lex_name(&mut self) -> Token {
    let text = self.consume_while(|c| c.is_ascii_alphanumeric());
    Token::Name(text)
  }

  fn skip_whitespace(&mut self) {
    self.consume_while(char::is_whitespace);
  }

  fn skip_trivial(&mut self) {
    self.skip_whitespace();
  }

  fn peek_one(&self) -> char {
    self.raw[self.cursor..].chars().next().unwrap()
  }

  fn peek_many(&self, count: usize) -> String {
    self.raw[self.cursor..].chars().take(count).collect()
  }

  fn starts_with(&self, s: &str) -> bool {
    self.raw[self.cursor..].starts_with(s)
  }

  fn is_end(&self) -> bool {
    self.cursor >= self.raw.len()
  }

  fn advance_one(&mut self) {
    self.cursor += 1;
  }

  fn advance_many(&mut self, count: usize) {
    self.cursor += count;
  }

  fn consume(&mut self) -> char {
    let mut iter = self.raw[self.cursor..].char_indices();
    let (_, cur_char) = iter.next().unwrap();
    let (next_cursor, _) = iter.next().unwrap_or((1, ' '));
    self.cursor += next_cursor;
    cur_char
  }

  fn consume_expect(&mut self, text: &str) {
    if &self.peek_many(text.len()) == text {
      self.advance_many(text.len());
    } else {
      panic!("Expected '{}' but got '{}'", text, &self.peek_many(text.len()));
    }
  }

  fn consume_while(&mut self, mut test: impl FnMut(char) -> bool) -> String {
    let start_cursor = self.cursor;
    while !self.is_end() && test(self.peek_one()) {
      self.advance_one();
    }
    self.raw[start_cursor..self.cursor].to_string()
  }
}
