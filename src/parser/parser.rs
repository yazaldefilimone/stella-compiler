use crate::{ast::Token, bytecode::ByteCode, lexer::Lexer, values::Value};

#[derive(Debug)]
pub struct Book {
  pub constants: Vec<Value>,
  pub byte_codes: Vec<ByteCode>,
}

#[derive(Debug)]
pub struct Parser {
  lex: Lexer,
}

impl Parser {
  pub fn new(raw: String) -> Self {
    let lex = Lexer::new(raw);
    Parser { lex }
  }

  pub fn parse_book(&mut self) -> Book {
    let mut constants = vec![];
    let mut byte_codes = vec![];
    loop {
      let token = self.lex.next_token();
      match token {
        Token::Name(name) => {
          constants.push(Value::String(name));
          let constant_index: u8 = (constants.len() - 1) as u8;
          byte_codes.push(ByteCode::GetGlobal(0, constant_index));
          // todo: remove this, it's just for testing purpose
          if let Token::String(string) = self.lex.next_token() {
            constants.push(Value::String(string));
            let constant_index: u8 = (constants.len() - 1) as u8;
            byte_codes.push(ByteCode::LoadConstant(1, constant_index));

            byte_codes.push(ByteCode::Call(0, 1));
          } else {
            panic!("Expected string after function name.");
          }
        }
        Token::String(string) => {
          constants.push(Value::String(string));
          let constant_index: u8 = (constants.len() - 1) as u8;
          byte_codes.push(ByteCode::GetGlobal(0, constant_index));
        }
        Token::Eof => break,

        _token => panic!("Unexpected token : {:?}", _token),
      }
    }
    // dbg!(&constants);
    // dbg!(&byte_codes);
    Book { constants, byte_codes }
  }
}
