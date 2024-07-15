// Copyright 2024 Yazalde Filimone <yazalde.filimone@gmail.com>
//
// Virtual Machine Execution
//

use std::collections::HashMap;

use crate::{stack::Stack, values::Value};

pub struct VirtualMachine {
  stack: Stack,
  globals: HashMap<String, Value>,
}
