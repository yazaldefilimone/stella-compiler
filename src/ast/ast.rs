// use super::Token;

// // Lua Abstract Syntax Tree

// #[derive(Debug, Clone)]
// pub enum LiteralExpression {
//   Identifier(Token),
//   Number(Token),
// }

// #[derive(Debug, Clone)]
// pub struct CallExpression {
//   pub name: Token,
//   pub arguments: Vec<Expression>,
// }

// #[derive(Debug, Clone)]
// pub struct BinaryOperation {
//   pub operator: Token,
//   pub left: Box<Expression>,
//   pub right: Box<Expression>,
// }

// #[derive(Debug, Clone)]
// pub struct VariableDeclaration {
//   pub name: Token,
//   pub expression: Box<Expression>,
// }

// #[derive(Debug, Clone)]
// pub struct FunctionDeclaration {
//   pub name: Token,
//   pub parameters: Vec<Token>,
//   pub body: Vec<Statement>,
// }

// #[derive(Debug, Clone)]
// pub struct ReturnExpression {
//   pub expression: Box<Expression>,
// }

// #[derive(Debug, Clone)]
// pub enum Expression {
//   BinaryOperation(BinaryOperation),
//   LiteralExpression(LiteralExpression),
//   VariableDeclaration(VariableDeclaration),
//   FunctionDeclaration(FunctionDeclaration),
//   CallExpression(CallExpression),
//   ReturnExpression(ReturnExpression),
// }

// #[derive(Debug, Clone)]
// pub struct IFExpression {
//   pub test: Expression,
//   pub body: Vec<Statement>,
// }

// #[derive(Debug, Clone)]
// pub struct Local {
//   pub name: Token,
//   pub expression: Expression,
// }

// #[derive(Debug, Clone)]
// pub struct Return {
//   pub expression: Expression,
// }

// // statements
// //
// //

// #[derive(Debug, Clone)]
// // while test do body end
// pub struct WhileStatement {
//   pub test: Expression,
//   pub body: Vec<Statement>,
// }

// #[derive(Debug, Clone)]
// // repeat body until test
// pub struct RepeatStatement {
//   pub body: Vec<Statement>,
//   pub test: Expression,
// }

// #[derive(Debug, Clone)]
// pub enum Statement {
//   Expression(Expression),
//   IFExpression(IFExpression),
//   FunctionDeclaration(FunctionDeclaration),
//   Return(Return),
//   Local(Local),
// }

// // Program

// #[derive(Debug, Clone)]
// pub struct Program {
//   pub statements: Vec<Statement>,
// }

// impl Program {
//   pub fn new(statements: Vec<Statement>) -> Self {
//     Program { statements }
//   }
// }
