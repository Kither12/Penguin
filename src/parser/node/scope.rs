use crate::{parser::ast::ASTNode, ProgramState};
use anyhow::Result;

use super::primitive::Primitive;

#[derive(Debug)]
pub struct Scope {
    pub code: Box<[ASTNode]>,
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

impl Scope {
    pub fn new(code: Box<[ASTNode]>) -> Self {
        Scope { code }
    }
    pub fn execute(
        &self,
        program: &ProgramState,
        is_function_scope: bool,
    ) -> Result<Option<FlowStatement>> {
        if is_function_scope == false {
            program.environment.borrow_mut().open_scope();
        }
        let mut flow_statement: Option<FlowStatement> = None;
        for node in self.code.iter() {
            match node {
                ASTNode::Expr(v) => v.execute(program).map(|_| ())?,
                ASTNode::Declaration(v) => v.execute(program)?,
                ASTNode::Assignment(v) => v.execute(program)?,
                ASTNode::Scope(v) => flow_statement = v.execute(program, false)?,
                ASTNode::IfElse(v) => flow_statement = v.execute(program)?,
                ASTNode::WhileLoop(v) => flow_statement = v.execute(program)?,
                ASTNode::Output(v) => v.execute(program)?,
                ASTNode::BreakStatement => flow_statement = Some(FlowStatement::Break),
                ASTNode::ContinueStatement => flow_statement = Some(FlowStatement::Continue),
                ASTNode::ReturnStatement(v) => {
                    let prim_value = v.execute(program)?;
                    flow_statement = Some(FlowStatement::Return(prim_value));
                }
            }
            if flow_statement.is_some() {
                break;
            }
        }
        if is_function_scope == false {
            program.environment.borrow_mut().close_scope();
        }
        Ok(flow_statement)
    }
}
