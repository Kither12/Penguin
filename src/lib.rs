use anyhow::{Ok, Result};
use environment::environment::Environment;
use parser::{ast::ASTNode, parser::parse_ast};

pub mod environment;
pub mod parser;

pub fn run_code(code: &str) -> Result<()> {
    let ast_root = parse_ast(code)?;
    let mut environment = Environment::default();
    if let ASTNode::Root(v) = ast_root.as_ref() {
        for node in v {
            match node.as_ref() {
                ASTNode::Expr(v) => {
                    println!("{:?}", v.evaluation(&environment));
                }
                ASTNode::Declaration(v) => {
                    environment = v.execute(environment)?;
                }
                ASTNode::Assignment(v) => {
                    environment = v.execute(environment)?;
                }
                _ => unreachable!(),
            }
        }
    }

    Ok(())
}
