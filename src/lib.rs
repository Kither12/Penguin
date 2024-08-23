use std::cell::RefCell;

use anyhow::{anyhow, Result};
use environment::environment::Environment;
use parser::{
    ast::ASTNode,
    node::{
        expression::ExpressionPool,
        primitive::Primitive,
        scope::{FlowStatement, ScopeError},
    },
    parser::parse_ast,
};

pub mod environment;
pub mod error;
pub mod parser;

pub struct ProgramState<'a> {
    expr_pool: ExpressionPool,
    environment: RefCell<Environment<'a>>,
}
impl<'a> ProgramState<'a> {
    pub fn new(expr_pool: ExpressionPool, environment: RefCell<Environment<'a>>) -> Self {
        ProgramState {
            expr_pool,
            environment,
        }
    }
}

pub fn run_code(code: &str) -> Result<()> {
    let (ast_root, mut program) = parse_ast(code)?;
    program.expr_pool.shrink();
    program.environment.borrow_mut().init();
    if let ASTNode::Scope(v) = ast_root {
        for node in v.code.iter() {
            let mut flow_statement: Option<FlowStatement> = None;
            match node {
                ASTNode::Expr(v) => v.execute(&program).map(|_| ())?,
                ASTNode::Declaration(v) => v.execute(&program)?,
                ASTNode::Assignment(v) => v.execute(&program)?,
                ASTNode::Scope(v) => flow_statement = v.execute(&program, false)?,
                ASTNode::IfElse(v) => flow_statement = v.execute(&program)?,
                ASTNode::WhileLoop(v) => flow_statement = v.execute(&program)?,
                ASTNode::Output(v) => v.execute(&program)?,
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
