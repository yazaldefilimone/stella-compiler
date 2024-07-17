#![allow(dead_code, unused_imports)]

use crate::utils::is_digit;

use super::token::Token;

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
      return Token::EOF;
    }

    let current_character = self.peek_one();
    match current_character {
      'a'..='z' | 'A'..='Z' | '_' => self.lex_identifier(),
      '"' => self.lex_string_literal(),
      '0'..='9' => self.lex_number_literal(),
      '(' => Token::LPAREN,
      ')' => Token::RPAREN,
      '{' => Token::LBRACE,
      '}' => Token::RBRACE,
      '[' => Token::LSQUARE,
      ']' => Token::RSQUARE,
      ':' => Token::COLON,
      ';' => Token::SEMICOLON,
      ',' => Token::COMMA,
      '.' => Token::DOT,
      '+' => Token::ADD,
      '-' => Token::SUB,
      '*' => Token::MUL,
      '/' => Token::DIV,
      '%' => Token::MOD,
      '^' => Token::POW,
      '#' => Token::LEN,
      '&' => Token::BITAND,
      '~' => Token::BITXOR,
      '|' => Token::BITOR,
      _ => panic!("Unexpected charcter '{}'", current_character),
    }
  }

  fn lex_string_literal(&mut self) -> Token {
    self.consume_expect("\"");
    let text = self.consume_while(|character| character != '"');
    self.consume_expect("\"");
    Token::String(text)
  }

  fn lex_number_literal(&mut self) -> Token {
    let text = self.consume_while(|character| is_digit(character));
    // todo: you can do better yazalde filimone :)
    match text.parse::<i64>() {
      Ok(integer) => Token::Integer(integer),
      Err(_) => Token::Float(text.parse::<f64>().unwrap()),
    }
  }

  fn lex_identifier(&mut self) -> Token {
    let text = self.consume_while(|c| c.is_ascii_alphanumeric());
    let token = self.consume_keyword_or_identifier(&text);
    return token;
  }

  fn consume_keyword_or_identifier(&mut self, text: &str) -> Token {
    match text {
      "and" => Token::AND,
      "break" => Token::BREAK,
      "do" => Token::DO,
      "else" => Token::ELSE,
      "elseif" => Token::ELSIF,
      "end" => Token::END,
      "false" => Token::FALSE,
      "for" => Token::FOR,
      "goto" => Token::GOTO,
      "if" => Token::IF,
      "in" => Token::IN,
      "local" => Token::LOCAL,
      "nil" => Token::NIL,
      "not" => Token::NOT,
      "or" => Token::OR,
      "repeat" => Token::REPEAT,
      "return" => Token::RETURN,
      "then" => Token::THEN,
      "true" => Token::TRUE,
      "until" => Token::UNTIL,
      "while" => Token::WHILE,
      "function" => Token::FUNCTION,
      _ => Token::Identifier(text.to_string()),
    }
  }
  // comment's
  fn lex_comment(&mut self) -> Token {
    self.consume_expect("--");
    let text = self.consume_while(|c| c != '\n');
    Token::Comment(text)
  }

  pub fn lex_block_comment(&mut self) -> Token {
    self.consume_expect("--[");
    let text = self.consume_while(|c| c != ']');
    self.consume_expect("]--");
    Token::BlockComment(text)
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
