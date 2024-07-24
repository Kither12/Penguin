use anyhow::{Context, Ok, Result};
use environment::environment::Environment;
use parser::{ast::ASTNode, parser::parse_ast};

pub mod environment;
pub mod parser;

pub fn run_code(code: &str) -> Result<()> {
    let ast_root = parse_ast(code)?;
    let mut environment = Environment::default();
    if let ASTNode::Scope(v) = ast_root.as_ref() {
        for node in v.code.iter() {
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
                        .context("Error found when try to run declaration")?;
                }
                ASTNode::Scope(v) => {
                    environment = v
                        .execute(environment)
                        .context("Error found when try to run declaration")?;
                }
                ASTNode::IfElse(v) => {
                    environment = v
                        .execute(environment)
                        .context("Error found when try to run declaration")?;
                }
            }
        }
    }
    Ok(())
}
