use crate::bytecode::{add_constant, load_constant};
use crate::lexer::Token;
use crate::{bytecode::ByteCode, lexer::Lexer, values::Value};

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
          let constant_index = add_constant(&mut constants, Value::String(name));
          byte_codes.push(ByteCode::GetGlobal(0, constant_index));
          match self.lex.peek_token() {
            Token::LPAREN => {
              self.lex.next_token(); // consume '('
              let byte_code = self.load_expression(&mut constants);
              // consume ')'
              if self.lex.next_token() != Token::RPAREN {
                panic!("Error: Expected ')' after arguments expression in function call.");
              }
              byte_codes.push(byte_code);
            }
            Token::String(_) => {
              let byte_code = self.load_expression(&mut constants);
              byte_codes.push(byte_code);
            }
            _ => {
              panic!("Expected '(' after function name.");
            }
          }

          byte_codes.push(ByteCode::Call(0, 1));

          // todo: remove this, it's just for testing purpose
          // if let Token::String(string) = self.lex.next_token() {
          //   constants.push(Value::String(string));
          //   let constant_index: u8 = (constants.len() - 1) as u8;
          //   byte_codes.push(ByteCode::LoadConstant(1, constant_index));

          // } else {
          // }
        }
        Token::String(string) => {
          let constant_index = add_constant(&mut constants, Value::String(string));
          byte_codes.push(ByteCode::GetGlobal(0, constant_index));
        }
        Token::LPAREN => {
          let byte_code = self.compile_function_call(&mut constants);
          byte_codes.push(byte_code);
          if let Token::RPAREN = self.lex.next_token() {
            continue;
          } else {
            panic!("Expected ')' after '('");
          }
        }
        Token::EOF => break,
        _token => panic!("Unexpected token : {:?}", _token),
      }
    }
    dbg!(&constants);
    dbg!(&byte_codes);
    Book { constants, byte_codes }
  }

  // call function expression
  //call( expression , expression )

  fn compile_function_call(&mut self, constants: &mut Vec<Value>) -> ByteCode {
    let token = self.lex.next_token();
    let byte_code = match token {
      Token::NIL => ByteCode::LoadNil(1),
      Token::TRUE => ByteCode::LoadBoolean(1, true),
      Token::FALSE => ByteCode::LoadBoolean(1, false),
      Token::Integer(integer) => {
        // as a bytecode has 4 bytes, of which the opcode occupies 1 byte,
        // the destination address occupies 1 byte and the rest 2 bytes,
        // which can be stored as a i16 number. Therefore, i16 numbers within the interval
        //
        if let Ok(integer) = i64::try_from(integer) {
          return ByteCode::LoadInteger(1, integer);
        }

        return load_constant(constants, 1, Value::Integer(integer));
      }
      Token::Float(float) => {
        return load_constant(constants, 1, Value::Float(float));
      }
      Token::String(string) => {
        return load_constant(constants, 1, Value::String(string));
      }

      _ => {
        panic!(" token : {:?}", token);
      }
    };
    return byte_code;
  }

  fn load_expression(&mut self, constants: &mut Vec<Value>) -> ByteCode {
    let token = self.lex.next_token();
    match token {
      Token::NIL => ByteCode::LoadNil(1),
      Token::TRUE => ByteCode::LoadBoolean(1, true),
      Token::FALSE => ByteCode::LoadBoolean(1, false),
      Token::Integer(integer) => {
        // as a bytecode has 4 bytes, of which the opcode occupies 1 byte,
        // the destination address occupies 1 byte and the rest 2 bytes,
        // which can be stored as a i16 number. Therefore, i16 numbers within the interval
        //
        if let Ok(integer) = i64::try_from(integer) {
          return ByteCode::LoadInteger(1, integer);
        }

        return load_constant(constants, 1, Value::Integer(integer));
      }
      Token::Float(float) => {
        return load_constant(constants, 1, Value::Float(float));
      }
      Token::String(string) => {
        return load_constant(constants, 1, Value::String(string));
      }
      _ => {
        panic!(" token : {:?}", token);
      }
    }
  }
}
