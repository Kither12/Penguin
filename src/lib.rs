use anyhow::{anyhow, Result};
use environment::environment::Environment;
use parser::{
    ast::ASTNode,
    node::{
        primitive::Primitive,
        scope::{FlowStatement, ScopeError},
    },
    parser::parse_ast,
};

pub mod environment;
pub mod error;
pub mod parser;

pub fn run_code(code: &str) -> Result<()> {
    let ast_root = parse_ast(code)?;
    let environment = Environment::default();
    if let ASTNode::Scope(v) = ast_root {
        for node in v.code.iter() {
            let mut flow_statement: Option<FlowStatement> = None;
            match node {
                ASTNode::Expr(v) => v.execute(&environment).map(|_| ())?,
                ASTNode::Declaration(v) => v.execute(&environment)?,
                ASTNode::Assignment(v) => v.execute(&environment)?,
                ASTNode::Scope(v) => flow_statement = v.execute(&environment, false)?,
                ASTNode::IfElse(v) => flow_statement = v.execute(&environment)?,
                ASTNode::WhileLoop(v) => flow_statement = v.execute(&environment)?,
                ASTNode::Output(v) => v.execute(&environment)?,
                ASTNode::BreakStatement => flow_statement = Some(FlowStatement::Break),
                ASTNode::ReturnStatement(_) => {
                    //flow statement here is only for error reporting so don't need to evaluate the expr inside it
                    flow_statement = Some(FlowStatement::Return(Primitive::VOID))
                }
                ASTNode::ContinueStatement => flow_statement = Some(FlowStatement::Continue),
            }
            match flow_statement {
                Some(FlowStatement::Break) => Err(anyhow!(ScopeError::BreakOutsideLoop))?,
                Some(FlowStatement::Return(_)) => Err(anyhow!(ScopeError::ReturnOutsideFunction))?,
                Some(FlowStatement::Continue) => Err(anyhow!(ScopeError::ContinueOutsideLoop))?,
                None => {}
            };
        }
    }
    Ok(())
}
