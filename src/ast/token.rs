use crate::utils::Location;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TokenKind {
  Identifier,
  Syntax,
  Keyword,
  Number,
  Operator,
}

#[derive(Debug, Clone)]
pub struct Token {
  pub value: String,
  pub kind: TokenKind,
  pub loc: Location,
}
