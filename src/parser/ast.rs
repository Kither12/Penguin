use super::node::{
    conditional::IfElse,
    declaration::{Assignment, Declaration},
    expression::Expr,
    io::Output,
    looping::WhileLoop,
    scope::Scope,
};

#[derive(Debug)]
pub enum ASTNode {
    Output(Output),
    Scope(Scope),
    Expr(Expr),
    Declaration(Declaration),
    Assignment(Assignment),
    IfElse(IfElse),
    WhileLoop(WhileLoop),
    BreakStatement,
    ReturnStatement(Expr),
    ContinueStatement,
}
