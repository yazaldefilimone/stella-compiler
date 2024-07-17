#![allow(dead_code)]

pub fn debug(raw: &[char], message: &str) -> String {
  let mut line = 1;
  let mut column = 0;
  let mut i = 0;
  while i < raw.len() {
    if raw[i] == '\n' {
      line += 1;
      column = 0;
    } else {
      column += 1;
    }
    i += 1;
  }
  format!(
    "{}:{}: {}\n{}",
    line,
    column,
    message,
    raw.iter().take(i).collect::<String>()
  )
}

pub fn is_digit(character: char) -> bool {
  return "0123456789".contains(character);
}

pub fn normalize_float_number(number: f64) -> String {
  if number.fract() == 0.0 {
    let integer = format!("{}", number);
    return integer.trim_end_matches(".0").to_string();
  }
  return format!("{}", number);
}
