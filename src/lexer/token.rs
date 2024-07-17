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
  LESEQ,     // <=
  GREEQ,     // >=
  LESS,      // <
  GREATER,   // >
  ASSIGN,    // =
  LPAREN,    // (
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
