use super::node::{
    conditional::IfElse,
    declaration::{Assignment, Declaration},
    expression::Expression,
    looping::WhileLoop,
    scope::Scope,
};

#[derive(Debug)]
pub enum ASTNode<'a> {
    Scope(Scope<'a>),
    Expr(Expression<'a>),
    Declaration(Declaration<'a>),
    Assignment(Assignment<'a>),
    IfElse(IfElse<'a>),
    WhileLoop(WhileLoop<'a>),
}
