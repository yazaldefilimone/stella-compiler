#![allow(dead_code)]

#[derive(Debug, PartialEq)]
pub enum Token {
  // keywords
  AND,
  BREAK,
  DO,
  ELSE,
  ELSIF,
  END,
  FALSE,
  FOR,
  GOTO,
  IF,
  IN,
  LOCAL,
  NIL,
  NOT,
  OR,
  REPEAT,
  RETURN,
  THEN,
  TRUE,
  UNTIL,
  WHILE,
  FUNCTION,
  // operators
  ADD,       // +
  SUB,       // -
  MUL,       // *
  DIV,       // /
  MOD,       // %
  POW,       // ^
  LEN,       // #
  BITAND,    // &
  BITXOR,    // ~
  BITOR,     // |
  SHIFTL,    // <<
  SHIFTR,    // >>
  IDIV,      // //
  EQUAL,     // ==
  NOTEQ,     // ~=
  LESSEQ,    // <=
  GREATEREQ, // >=
  LESS,      // <
  GREATER,   // >
  ASSIGN,    // =
  LPAREN,    //
  RPAREN,    // )
  LBRACE,    // {
  RBRACE,    // }
  LSQUARE,   // [
  RSQUARE,   // ]
  DOUBCOLON, // ::
  SEMICOLON, // ;
  COLON,     // :
  COMMA,     // ,
  DOT,       // .
  CONCAT,    // ..
  DOTS,      // ...

  // Comment
  Comment(String),
  BlockComment(String),
  // constant values
  Integer(i64),
  Float(f64),
  String(String),
  // name of variables or table keys
  Identifier(String),
  // end of file
  EOF,
}

impl Token {
  pub fn lookup_identifier(raw: &str) -> Token {
    match raw.trim() {
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
      current => Token::Identifier(current.to_string()),
    }
  }

  pub fn get_operator(raw: &str) -> Option<Token> {
    let operator = match raw.trim() {
      "+" => Token::ADD,
      "-" => Token::SUB,
      "*" => Token::MUL,
      "/" => Token::DIV,
      "%" => Token::MOD,
      "^" => Token::POW,
      "#" => Token::LEN,
      "&" => Token::BITAND,
      "~" => Token::BITXOR,
      "|" => Token::BITOR,
      "<" => Token::LESS,
      ">" => Token::GREATER,
      "=" => Token::ASSIGN,
      "(" => Token::LPAREN,
      ")" => Token::RPAREN,
      "{" => Token::LBRACE,
      "}" => Token::RBRACE,
      "[" => Token::LSQUARE,
      "]" => Token::RSQUARE,
      "::" => Token::DOUBCOLON,
      ";" => Token::SEMICOLON,
      ":" => Token::COLON,
      "," => Token::COMMA,
      "." => Token::DOT,
      ".." => Token::CONCAT,
      "..." => Token::DOTS,
      _ => return None,
    };
    return Some(operator);
  }
}

// ---  tests  ---

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_get_keyword() {
    assert_eq!(Token::lookup_identifier("and"), Token::AND);
    assert_eq!(Token::lookup_identifier("break"), Token::BREAK);
    assert_eq!(Token::lookup_identifier("do"), Token::DO);
    assert_eq!(Token::lookup_identifier("else"), Token::ELSE);
    assert_eq!(Token::lookup_identifier("elseif"), Token::ELSIF);
    assert_eq!(Token::lookup_identifier("end"), Token::END);
    assert_eq!(Token::lookup_identifier("false"), Token::FALSE);
    assert_eq!(Token::lookup_identifier("for"), Token::FOR);
    assert_eq!(Token::lookup_identifier("goto"), Token::GOTO);
    assert_eq!(Token::lookup_identifier("if"), Token::IF);
    assert_eq!(Token::lookup_identifier("in"), Token::IN);
    assert_eq!(Token::lookup_identifier("local"), Token::LOCAL);
    assert_eq!(Token::lookup_identifier("nil"), Token::NIL);
    assert_eq!(Token::lookup_identifier("not"), Token::NOT);
    assert_eq!(Token::lookup_identifier("or"), Token::OR);
    assert_eq!(Token::lookup_identifier("repeat"), Token::REPEAT);
    assert_eq!(Token::lookup_identifier("return"), Token::RETURN);
    assert_eq!(Token::lookup_identifier("then"), Token::THEN);
    assert_eq!(Token::lookup_identifier("true"), Token::TRUE);
    assert_eq!(Token::lookup_identifier("until"), Token::UNTIL);
    assert_eq!(Token::lookup_identifier("while"), Token::WHILE);
    assert_eq!(Token::lookup_identifier("function"), Token::FUNCTION);
  }

  #[test]
  fn test_get_operator() {
    assert_eq!(Token::get_operator("+"), Some(Token::ADD));
    assert_eq!(Token::get_operator("-"), Some(Token::SUB));
    assert_eq!(Token::get_operator("*"), Some(Token::MUL));
    assert_eq!(Token::get_operator("/"), Some(Token::DIV));
    assert_eq!(Token::get_operator("%"), Some(Token::MOD));
    assert_eq!(Token::get_operator("^"), Some(Token::POW));
    assert_eq!(Token::get_operator("#"), Some(Token::LEN));
    assert_eq!(Token::get_operator("&"), Some(Token::BITAND));
    assert_eq!(Token::get_operator("~"), Some(Token::BITXOR));
    assert_eq!(Token::get_operator("|"), Some(Token::BITOR));
    assert_eq!(Token::get_operator("<"), Some(Token::LESS));
    assert_eq!(Token::get_operator(">"), Some(Token::GREATER));
    assert_eq!(Token::get_operator("="), Some(Token::ASSIGN));
    assert_eq!(Token::get_operator("("), Some(Token::LPAREN));
    assert_eq!(Token::get_operator(")"), Some(Token::RPAREN));
    assert_eq!(Token::get_operator("{"), Some(Token::LBRACE));
    assert_eq!(Token::get_operator("}"), Some(Token::RBRACE));
    assert_eq!(Token::get_operator("["), Some(Token::LSQUARE));
    assert_eq!(Token::get_operator("]"), Some(Token::RSQUARE));
    assert_eq!(Token::get_operator("::"), Some(Token::DOUBCOLON));
    assert_eq!(Token::get_operator(";"), Some(Token::SEMICOLON));
    assert_eq!(Token::get_operator(":"), Some(Token::COLON));
    assert_eq!(Token::get_operator(","), Some(Token::COMMA));
    assert_eq!(Token::get_operator("."), Some(Token::DOT));
    assert_eq!(Token::get_operator(".."), Some(Token::CONCAT));
    assert_eq!(Token::get_operator("..."), Some(Token::DOTS));
  }
}
