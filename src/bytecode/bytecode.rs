#[derive(Debug)]
pub enum ByteCode {
  LoadConstant(u8, u8),
  GetGlobal(u8, u8),
  Call(u8, u8),
}
