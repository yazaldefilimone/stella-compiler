use crate::{ast, utils::syntax::expect_syntax};

fn parse_statement(raw: &[char], tokens: &[ast::Token], index: usize) -> Option<(ast::Statement, usize)> {
  let parsers = [
    parse_if,
    parse_expression_statement,
    parse_return,
    parse_function,
    parse_local,
  ];
  for parser in parsers {
    let res = parser(raw, tokens, index);
    if res.is_some() {
      return res;
    }
  }

  None
}

pub fn parse(raw: &[char], tokens: Vec<ast::Token>) -> Result<ast::Program, String> {
  let mut ast = vec![];
  let mut index = 0;
  let ntokens = tokens.len();
  while index < ntokens {
    let res = parse_statement(raw, &tokens, index);
    if let Some((stmt, next_index)) = res {
      index = next_index;
      ast.push(stmt);
      continue;
    }

    return Err(tokens[index].loc.debug(raw, "Invalid token while parsing:"));
  }

  Ok(ast)
}

fn parse_expression_statement(raw: &[char], tokens: &[ast::Token], index: usize) -> Option<(ast::Statement, usize)> {
  let mut next_index = index;
  let res = parse_expression(raw, tokens, next_index)?;

  let (expr, next_next_index) = res;
  next_index = next_next_index;
  if !expect_syntax(tokens, next_index, ";") {
    let error = tokens[next_index]
      .loc
      .debug(raw, "Expected semicolon after expression:");

    println!("{:?}", error);
    return None;
  }

  next_index += 1; // Skip past semicolon

  Some((Statement::Expression(expr), next_index))
}

fn parse_expression(raw: &[char], tokens: &[ast::Token], index: usize) -> Option<(ast::Expression, usize)> {
  let mut next_index = index;
  let res = parse_assignment(raw, tokens, next_index)?;

  let (expr, next_next_index) = res;
  next_index = next_next_index;

  Some((expr, next_index))
}

fn parse_assignment(raw: &[char], tokens: &[ast::Token], index: usize) -> Option<(ast::Expression, usize)> {
  let mut next_index = index;
  let res = parse_if(raw, tokens, next_index)?;

  let (expr, next_next_index) = res;
  next_index = next_next_index;

  Some((expr, next_index))
}
