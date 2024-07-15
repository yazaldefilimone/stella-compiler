// Copyright 2024 Yazalde Filimone <yazalde.filimone@gmail.com>
//
// Base library
//

use crate::vm::ExeState;

pub fn lib_print(state: &mut ExeState) -> i32 {
  println!("{}", state.stack.get(1));
  return 0;
}
