use crate::{environment::environment::Environment, parser::ast::ASTNode};
use anyhow::{Context, Result};

#[derive(Debug)]
pub struct Scope<'a> {
    pub code: Vec<Box<ASTNode<'a>>>,
}

impl<'a> Scope<'a> {
    pub fn new(code: Vec<Box<ASTNode<'a>>>) -> Self {
        Scope { code }
    }
    pub fn execute(&'a self, mut environment: Environment<'a>) -> Result<Environment> {
        environment = environment.open_scope();
        for node in self.code.iter() {
            match node.as_ref() {
                ASTNode::Expr(v) => {
                    println!(
                        "{:?}",
                        v.evaluation(&environment)
                            .context("Error found when try to run expression")?
                    );
                }
                ASTNode::Declaration(v) => {
                    environment = v
                        .execute(environment)
                        .context("Error found when try to run declaration")?;
                }
                ASTNode::Assignment(v) => {
                    environment = v
                        .execute(environment)
                        .context("Error found when try to run assignment")?;
                }
                ASTNode::Scope(v) => {
                    environment = v
                        .execute(environment)
                        .context("Error found when try to run scope")?;
                }
                ASTNode::IfElse(v) => {
                    environment = v
                        .execute(environment)
                        .context("Error found when try to run declaration")?;
                }
            }
        }
        environment = environment.close_scope();

        Ok(environment)
    }
}
