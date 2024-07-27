use anyhow::{Context, Ok, Result};
use environment::environment::Environment;
use parser::{ast::ASTNode, parser::parse_ast};

pub mod environment;
pub mod error;
pub mod parser;

pub fn run_code(code: &str) -> Result<()> {
    let ast_root = parse_ast(code)?;
    let mut environment = Environment::default();
    if let ASTNode::Scope(v) = ast_root {
        for node in v.code.iter() {
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
    }
    Ok(())
}
