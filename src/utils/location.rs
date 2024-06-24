#[derive(Debug)]
pub struct Location {
  column: usize,
  line: usize,
}

impl Default for Location {
  fn default() -> Self {
    Location { line: 1, column: 0 }
  }
}

impl Location {
  pub fn new(line: usize, column: usize) -> Self {
    Location { line, column }
  }
  pub fn walk(&mut self, is_newline: bool) {
    if is_newline {
      self.line += 1;
      self.column = 0;
    } else {
      self.column += 1;
    }
  }

  pub fn get_current_location(&self) -> Location {
    Location { line: self.line, column: self.column }
  }
  pub fn get_line(&self) -> usize {
    self.line
  }
  pub fn get_column(&self) -> usize {
    self.column
  }

  // 1:54 SyntaxError: unexpected symbol near '='
  pub fn debug(&mut self, raw: &[char], message: &str) -> String {
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
}
