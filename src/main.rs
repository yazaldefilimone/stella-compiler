use parser::Parser;
use vm::ExeState;

mod ast;
mod bytecode;
mod format;
mod lexer;
mod parser;
mod stack;
mod stdlib;
mod utils;
mod values;
mod vm;

mod cli;

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
  let mut parser = Parser::new(raw);
  let book = parser.parse_book();
  let mut exe_state = ExeState::new();
  exe_state.execute(&book);
}

fn compile_lua(file: &str) {
  let contents = std::fs::read_to_string(file).unwrap();
  println!("{}", contents);
}
