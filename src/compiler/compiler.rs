use crate::bytecode::load_constant;
use crate::lexer::Token;
use crate::{bytecode::ByteCode, lexer::Lexer, values::Value};

#[derive(Debug)]
pub struct Book {
  pub constants: Vec<Value>,
  pub byte_codes: Vec<ByteCode>,
}

#[derive(Debug)]
pub struct Compiler {
  lex: Lexer,
}

impl Compiler {
  pub fn new(raw: String) -> Self {
    let lex = Lexer::new(raw);
    Compiler { lex }
  }

  pub fn compile_book(&mut self) -> Book {
    let mut constants = vec![];
    let mut byte_codes = vec![];
    loop {
      let token = self.lex.next_token();
      match token {
        Token::Identifier(name) => {
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
        Token::LPAREN => {
          match self.lex.next_token() {
            Token::RPAREN => {
              let byte_code = self.compile_call_expression(&mut constants);
              byte_codes.push(byte_code);
            }
            _ => {
              panic!("Expected ')' after '('");
            }
          }
        }
        Token::EOF => break,
        _token => panic!("Unexpected token : {:?}", _token),
      }
    }
    // dbg!(&constants);
    // dbg!(&byte_codes);
    Book { constants, byte_codes }
  }

  // call function expression
  // function_name ( expression , expression )

  fn compile_call_expression(&mut self, constants: &mut Vec<Value>) -> ByteCode {
    let expression = self.lex.next_token();
    match expression {
      Token::NIL => ByteCode::LoadNil(1),
      Token::TRUE => ByteCode::LoadBoolean(1, true),
      Token::FALSE => ByteCode::LoadBoolean(1, false),
      Token::Integer(integer) =>  {
        if let Ok(integer) = i64::try_from(integer) {
          return ByteCode::LoadInteger(1, integer);
        }
        return load_constant(constants, 1, Value::Integer(integer));
      },
      Token::Float(float) => {
        return load_constant(constants, 1, Value::Float(float));
      },
      Token::String(string) => {
        return load_constant(constants, 1, Value::String(string));
      },
      _ => {
        panic!("Unexpected token : {:?}", expression),
      }
    }
    return ByteCode::Nop;
  }
}
