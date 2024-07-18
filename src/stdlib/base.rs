// Copyright 2024 Yazalde Filimone <yazalde.filimone@gmail.com>
//
// Base library
//

use crate::vm::VirtualMachine;

pub fn lib_print(state: &mut VirtualMachine) -> i32 {
  println!("{:?}", state.stack.get(1));
  return 0;
}
