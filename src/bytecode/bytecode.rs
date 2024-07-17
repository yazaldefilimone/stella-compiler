#[derive(Debug)]
pub enum ByteCode {
  LoadConstant(u8, u8),
  GetGlobal(u8, u8),
  Call(u8, u8),
  LoadBoolean(u8, bool),
  LoadInteger(u8, i64),
  LoadFloat(u8, f64),
  LoadNil(u8),
  Nop,
}
