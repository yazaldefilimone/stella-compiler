use super::Token;

#[derive(Debug)]
pub enum Literal {
  Identifier(Token),
  Number(Token),
}

#[derive(Debug)]
pub struct FunctionCall {
  pub name: Token,
  pub arguments: Vec<Expression>,
}

#[derive(Debug)]
pub struct BinaryOperation {
  pub operator: Token,
  pub left: Box<Expression>,
  pub right: Box<Expression>,
}

#[derive(Debug)]
pub enum Expression {
  FunctionCall(FunctionCall),
  BinaryOperation(BinaryOperation),
  Literal(Literal),
}

#[derive(Debug)]
pub struct FunctionDeclaration {
  pub name: Token,
  pub parameters: Vec<Token>,
  pub body: Vec<Statement>,
}

#[derive(Debug)]
pub struct IFExpression {
  pub test: Expression,
  pub body: Vec<Statement>,
}

#[derive(Debug)]
pub struct Local {
  pub name: Token,
  pub expression: Expression,
}

#[derive(Debug)]
pub struct Return {
  pub expression: Expression,
}

#[derive(Debug)]
pub enum Statement {
  Expression(Expression),
  IFExpression(IFExpression),
  FunctionDeclaration(FunctionDeclaration),
  Return(Return),
  Local(Local),
}

pub type Program = Vec<Statement>;
