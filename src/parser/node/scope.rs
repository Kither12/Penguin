use crate::{environment::environment::Environment, parser::ast::ASTNode};
use anyhow::Result;

#[derive(Debug)]
pub struct Scope<'a> {
    pub code: Vec<ASTNode<'a>>,
}

impl<'a> Scope<'a> {
    pub fn new(code: Vec<ASTNode<'a>>) -> Self {
        Scope { code }
    }
    pub fn execute(&'a self, mut environment: Environment<'a>) -> Result<Environment> {
        environment = environment.open_scope();
        for node in self.code.iter() {
            match node {
                ASTNode::Expr(v) => {
                    println!("{:?}", v.evaluation(&environment)?);
                }
                ASTNode::Declaration(v) => {
                    environment = v.execute(environment)?;
                }
                ASTNode::Assignment(v) => {
                    environment = v.execute(environment)?;
                }
                ASTNode::Scope(v) => {
                    environment = v.execute(environment)?;
                }
                ASTNode::IfElse(v) => {
                    environment = v.execute(environment)?;
                }
                ASTNode::WhileLoop(v) => {
                    environment = v.execute(environment)?;
                }
            }
        }
        environment = environment.close_scope();

        Ok(environment)
    }
}
