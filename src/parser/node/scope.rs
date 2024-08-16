use crate::{environment::environment::Environment, parser::ast::ASTNode};
use anyhow::Result;

use super::primitive::Primitive;

#[derive(Debug)]
pub struct Scope<'a> {
    pub code: Vec<ASTNode<'a>>,
}

#[derive(Debug)]
pub enum ScopeError {
    ReturnOutsideFunction,
    ContinueOutsideLoop,
    BreakOutsideLoop,
}

pub enum FlowStatement {
    Continue,
    Break,
    Return(Primitive),
}

impl std::fmt::Display for ScopeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ReturnOutsideFunction => {
                write!(f, "a return statement may only be used within a function")
            }
            Self::ContinueOutsideLoop => {
                write!(f, "a continue statement may only be used within a loop")
            }
            Self::BreakOutsideLoop => {
                write!(f, "a break statement may only be used within a loop")
            }
        }
    }
}

impl<'a> Scope<'a> {
    pub fn new(code: Vec<ASTNode<'a>>) -> Self {
        Scope { code }
    }
    pub fn execute(&'a self, environment: &Environment<'a>) -> Result<Option<FlowStatement>> {
        environment.open_scope();
        let mut flow_statement: Option<FlowStatement> = None;
        for node in self.code.iter() {
            match node {
                ASTNode::Expr(v) => v.execute(environment).map(|_| ())?,
                ASTNode::Declaration(v) => v.execute(environment)?,
                ASTNode::Assignment(v) => v.execute(environment)?,
                ASTNode::Scope(v) => flow_statement = v.execute(environment)?,
                ASTNode::IfElse(v) => flow_statement = v.execute(environment)?,
                ASTNode::WhileLoop(v) => flow_statement = v.execute(environment)?,
                ASTNode::Output(v) => v.execute(environment)?,
                ASTNode::BreakStatement => flow_statement = Some(FlowStatement::Break),
                ASTNode::ContinueStatement => flow_statement = Some(FlowStatement::Continue),
                ASTNode::ReturnStatement(v) => {
                    let prim_value = v.execute(environment)?;
                    flow_statement = Some(FlowStatement::Return(prim_value));
                }
            }
            if flow_statement.is_some() {
                break;
            }
        }
        environment.close_scope();

        Ok(flow_statement)
    }
}
