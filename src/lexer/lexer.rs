#![allow(dead_code, unused_imports)]

use crate::utils::is_digit;

use super::token::Token;

#[derive(Debug)]
pub struct Lexer {
  raw: String,
  cursor: usize,
  peeked_token: Option<Token>,
  peeked: bool,
}

impl Lexer {
  pub fn new(raw: String) -> Self {
    Lexer { raw, cursor: 0, peeked_token: None, peeked: false }
  }

  pub fn peek_token(&mut self) -> Token {
    let next_token = self.read_next_token();
    self.peeked = true;
    self.peeked_token = Some(next_token.clone());
    return next_token;
  }

  pub fn next_token(&mut self) -> Token {
    if self.peeked_token.is_some() && self.peeked {
      self.peeked = false;
      return self.peeked_token.clone().unwrap();
    }
    self.peeked = false;
    self.peeked_token = None;
    return self.read_next_token();
  }

  fn read_next_token(&mut self) -> Token {
    self.skip_whitespace();
    if self.is_end() {
      return Token::EOF;
    }
    let current_character = self.peek_one();
    match current_character {
      'a'..='z' | 'A'..='Z' | '_' => self.read_identifier(),
      '"' => self.read_string(),
      '0'..='9' => self.read_number(),
      '<' => self.read_less_or_less_equal(),
      '>' => self.read_greater_or_greater_equal(),
      '~' => self.read_bitxor_or_noteq(),
      '.' => self.read_dot_or_concat(),
      '=' => self.read_equal_or_assign(),
      '+' => self.create_token(Token::ADD, 1),
      '-' => self.create_token(Token::SUB, 1),
      '*' => self.create_token(Token::MUL, 1),
      '/' => self.create_token(Token::DIV, 1),
      '%' => self.create_token(Token::MOD, 1),
      '^' => self.create_token(Token::POW, 1),
      '#' => self.create_token(Token::LEN, 1),
      '(' => self.create_token(Token::LPAREN, 1),
      ')' => self.create_token(Token::RPAREN, 1),
      '{' => self.create_token(Token::LBRACE, 1),
      '}' => self.create_token(Token::RBRACE, 1),
      '[' => self.create_token(Token::LSQUARE, 1),
      ']' => self.create_token(Token::RSQUARE, 1),
      ':' => self.create_token(Token::COLON, 1),
      ';' => self.create_token(Token::SEMICOLON, 1),
      ',' => self.create_token(Token::COMMA, 1),
      '&' => self.create_token(Token::BITAND, 1),
      '|' => self.create_token(Token::BITOR, 1),
      _ => panic!("Unexpected charcter '{}'", current_character),
    }
  }

  fn read_less_or_less_equal(&mut self) -> Token {
    match self.peek_many(2).as_str() {
      "<=" => self.create_token(Token::LESSEQ, 2),
      _ => {
        self.advance_one();
        return Token::LESS;
      }
    }
  }

  fn read_greater_or_greater_equal(&mut self) -> Token {
    match self.peek_many(2).as_str() {
      ">=" => self.create_token(Token::GREATEREQ, 2),
      _ => {
        self.advance_one();
        return Token::GREATER;
      }
    }
  }
  fn read_dot_or_concat(&mut self) -> Token {
    match self.peek_many(2).as_str() {
      ".." => self.create_token(Token::CONCAT, 2),
      _ => {
        self.advance_one();
        return Token::DOT;
      }
    }
  }
  fn read_equal_or_assign(&mut self) -> Token {
    match self.peek_many(2).as_str() {
      "==" => self.create_token(Token::EQUAL, 2),
      _ => {
        self.advance_one();
        return Token::ASSIGN;
      }
    }
  }
  fn read_bitxor_or_noteq(&mut self) -> Token {
    match self.peek_many(2).as_str() {
      "~=" => self.create_token(Token::NOTEQ, 2),
      _ => {
        self.advance_one();
        return Token::BITXOR;
      }
    }
  }

  fn read_bitand(&mut self) -> Token {
    self.advance_one();
    Token::BITAND
  }

  fn read_bitor(&mut self) -> Token {
    self.advance_one();
    Token::BITOR
  }

  fn read_string(&mut self) -> Token {
    self.consume_expect("\"");
    let text = self.consume_while(|character| character != '"');
    self.consume_expect("\"");
    Token::String(text)
  }

  fn read_number(&mut self) -> Token {
    let text = self.consume_while(|character| is_digit(character) || character == '.');

    // if the number is a float
    if text.contains(".") {
      return Token::Float(text.parse::<f64>().unwrap());
    }
    // if the number is an integer
    // todo: you can do better yazalde filimone :)
    match text.parse::<i64>() {
      Ok(integer) => Token::Integer(integer),
      Err(_) => Token::Float(text.parse::<f64>().unwrap()),
    }
  }

  fn read_identifier(&mut self) -> Token {
    let text = self.consume_while(|c| c.is_ascii_alphanumeric());
    Token::lookup_identifier(&text)
  }

  fn create_token(&mut self, token: Token, steps: usize) -> Token {
    self.advance_many(steps);
    return token;
  }

  // comment's
  fn read_comment(&mut self) -> Token {
    self.consume_expect("--");
    let text = self.consume_while(|c| c != '\n');
    Token::Comment(text)
  }

  pub fn read_block_comment(&mut self) -> Token {
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

// todo: I don't like to write tests in same file, please yazalde filimone move this to /tests folder :)
#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_lexer() {
    let raw = r#"
      local a = 1;

      print(a);
      print(nil);
      print(false);

      print(123);
      print(-123);
      print(12.35);

      print("hello world");

      print(a ~= 1);
      print(a <= 1);
      print(a >= 1);

      "#;

    let mut lexer = Lexer::new(raw.to_string());
    let token = lexer.next_token();
    assert_eq!(token, Token::LOCAL);
    let token = lexer.next_token();
    assert_eq!(token, Token::Identifier("a".to_string()));
    let token = lexer.next_token();
    assert_eq!(token, Token::ASSIGN);
    let token = lexer.next_token();
    assert_eq!(token, Token::Integer(1));
    let token = lexer.next_token();
    assert_eq!(token, Token::SEMICOLON);
    // print(a);
    let token = lexer.next_token();
    assert_eq!(token, Token::Identifier("print".to_string()));
    let token = lexer.next_token();
    assert_eq!(token, Token::LPAREN);
    let token = lexer.next_token();
    assert_eq!(token, Token::Identifier("a".to_string()));
    let token = lexer.next_token();
    assert_eq!(token, Token::RPAREN);
    let token = lexer.next_token();
    assert_eq!(token, Token::SEMICOLON);
    // print(nil);
    let token = lexer.next_token();
    assert_eq!(token, Token::Identifier("print".to_string()));
    let token = lexer.next_token();
    assert_eq!(token, Token::LPAREN);
    let token = lexer.next_token();
    assert_eq!(token, Token::NIL);
    let token = lexer.next_token();
    assert_eq!(token, Token::RPAREN);
    let token = lexer.next_token();
    assert_eq!(token, Token::SEMICOLON);

    // print(false);
    let token = lexer.next_token();
    assert_eq!(token, Token::Identifier("print".to_string()));
    let token = lexer.next_token();
    assert_eq!(token, Token::LPAREN);
    let token = lexer.next_token();
    assert_eq!(token, Token::FALSE);
    let token = lexer.next_token();
    assert_eq!(token, Token::RPAREN);
    let token = lexer.next_token();
    assert_eq!(token, Token::SEMICOLON);

    //

    // print(123);
    let token = lexer.next_token();
    assert_eq!(token, Token::Identifier("print".to_string()));
    let token = lexer.next_token();
    assert_eq!(token, Token::LPAREN);
    let token = lexer.next_token();
    assert_eq!(token, Token::Integer(123));
    let token = lexer.next_token();
    assert_eq!(token, Token::RPAREN);
    let token = lexer.next_token();
    assert_eq!(token, Token::SEMICOLON);

    // print(-123);
    let token = lexer.next_token();
    assert_eq!(token, Token::Identifier("print".to_string()));
    let token = lexer.next_token();
    assert_eq!(token, Token::LPAREN);
    let token = lexer.next_token();
    assert_eq!(token, Token::SUB);
    let token = lexer.next_token();
    assert_eq!(token, Token::Integer(123));
    let token = lexer.next_token();
    assert_eq!(token, Token::RPAREN);
    let token = lexer.next_token();
    assert_eq!(token, Token::SEMICOLON);

    // print(12.35);
    // [print(12.35);
    let token = lexer.next_token();
    assert_eq!(token, Token::Identifier("print".to_string()));
    let token = lexer.next_token();
    assert_eq!(token, Token::LPAREN);
    let token = lexer.next_token();
    assert_eq!(token, Token::Float(12.35));
    let token = lexer.next_token();
    assert_eq!(token, Token::RPAREN);
    let token = lexer.next_token();
    assert_eq!(token, Token::SEMICOLON);

    // print("hello world");
    let token = lexer.next_token();
    assert_eq!(token, Token::Identifier("print".to_string()));
    let token = lexer.next_token();
    assert_eq!(token, Token::LPAREN);
    let token = lexer.next_token();
    match token {
      Token::String(string) => assert_eq!(string, "hello world"),
      _ => panic!("expected string"),
    }
    let token = lexer.next_token();
    assert_eq!(token, Token::RPAREN);
    let token = lexer.next_token();
    assert_eq!(token, Token::SEMICOLON);

    // print(a ~= 1);
    let token = lexer.next_token();
    assert_eq!(token, Token::Identifier("print".to_string()));
    let token = lexer.next_token();
    assert_eq!(token, Token::LPAREN);
    let token = lexer.next_token();
    assert_eq!(token, Token::Identifier("a".to_string()));
    let token = lexer.next_token();
    assert_eq!(token, Token::NOTEQ);
    let token = lexer.next_token();
    assert_eq!(token, Token::Integer(1));
    let token = lexer.next_token();
    assert_eq!(token, Token::RPAREN);
    let token = lexer.next_token();
    assert_eq!(token, Token::SEMICOLON);

    // print(a <= 1);
    let token = lexer.next_token();
    assert_eq!(token, Token::Identifier("print".to_string()));
    let token = lexer.next_token();
    assert_eq!(token, Token::LPAREN);
    let token = lexer.next_token();
    assert_eq!(token, Token::Identifier("a".to_string()));
    let token = lexer.next_token();
    assert_eq!(token, Token::LESSEQ);
    let token = lexer.next_token();
    assert_eq!(token, Token::Integer(1));
    let token = lexer.next_token();
    assert_eq!(token, Token::RPAREN);
    let token = lexer.next_token();
    assert_eq!(token, Token::SEMICOLON);

    // print(a >= 1);
    let token = lexer.next_token();
    assert_eq!(token, Token::Identifier("print".to_string()));
    let token = lexer.next_token();
    assert_eq!(token, Token::LPAREN);
    let token = lexer.next_token();
    assert_eq!(token, Token::Identifier("a".to_string()));
    let token = lexer.next_token();
    assert_eq!(token, Token::GREATEREQ);
    let token = lexer.next_token();
    assert_eq!(token, Token::Integer(1));
    let token = lexer.next_token();
    assert_eq!(token, Token::RPAREN);
    let token = lexer.next_token();
    assert_eq!(token, Token::SEMICOLON);
    //end of file
    let token = lexer.next_token();
    assert_eq!(token, Token::EOF);
  }

  // test peek token
  #[test]
  fn test_lexer_peek_token() {
    let raw = r#"
      print(a >= 1);
      "#;

    let mut lexer = Lexer::new(raw.to_string());
    let token = lexer.next_token();
    assert_eq!(token, Token::Identifier("print".to_string()));
    let token = lexer.peek_token();
    assert_eq!(token, Token::LPAREN);
    let token = lexer.next_token();
    assert_eq!(token, Token::LPAREN);
    let token = lexer.next_token();
    assert_eq!(token, Token::Identifier("a".to_string()));
    let token = lexer.next_token();
    assert_eq!(token, Token::GREATEREQ);
    let token = lexer.next_token();
    assert_eq!(token, Token::Integer(1));
    let token = lexer.next_token();
    assert_eq!(token, Token::RPAREN);
    let token = lexer.next_token();
    assert_eq!(token, Token::SEMICOLON);
  }
}
