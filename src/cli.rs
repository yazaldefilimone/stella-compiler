// Copyright 2024 Yazalde Filimone <yazalde.filimone@gmail.com>
//
// Virtual Machine Command Line Interface
//

use clap::{Arg, Command};

pub fn command_line() -> clap::ArgMatches {
  let matches = Command::new("Lua")
    .about("Rust-based, speedy, lightweight lua virtual machine.")
    .subcommand_required(true)
    .arg_required_else_help(true)
    .author("yazaldefi <yazaldefilimon@gmail.com>")
    .subcommand(
      Command::new("run")
        .about("execute a lua file.")
        .arg(Arg::new("file").help("the lua file to execute.").required(true)),
    )
    .subcommand(
      Command::new("compile")
        .about("compile a lua file to bytecode.")
        .arg(Arg::new("file").help("the lua file to compile.").required(true)),
    )
    .get_matches();

  return matches;
}
