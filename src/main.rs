use compiler::Compiler;
use vm::ExeState;

mod bytecode;
mod cli;
mod compiler;
mod diagnostics;
mod format;
mod lexer;
mod stack;
mod stdlib;
mod utils;
mod values;
mod vm;

fn main() {
  let matches = cli::command_line();

  match matches.subcommand() {
    Some(("run", matches)) => {
      let file = matches.get_one::<String>("file").unwrap();
      run_lua(file);
    }
    Some(("compile", matches)) => {
      let file = matches.get_one::<String>("file").unwrap();
      compile_lua(file);
    }
    _ => println!("No command provided."),
  }
}

fn run_lua(file: &str) {
  let raw = std::fs::read_to_string(file).unwrap();
  let mut compiler = Compiler::new(raw);
  let book = compiler.compile_book();
  let mut exe_state = ExeState::new();
  exe_state.execute(&book);
}

fn compile_lua(file: &str) {
  let contents = std::fs::read_to_string(file).unwrap();
  println!("{}", contents);
}
