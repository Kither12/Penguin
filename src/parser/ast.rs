use super::node::{
    declaration::{Assignment, Declaration},
    expression::Expression,
};
#[derive(Debug)]
pub enum ASTNode<'a> {
    Root(Vec<Box<ASTNode<'a>>>),
    Expr(Expression<'a>),
    Declaration(Declaration<'a>),
    Assignment(Assignment<'a>),
}
