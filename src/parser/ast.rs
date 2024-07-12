use super::node::expression::Expression;
#[derive(Debug)]
pub enum ASTNode {
    Expr(Expression),
}
