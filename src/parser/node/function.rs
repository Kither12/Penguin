use std::rc::Rc;

use crate::{environment::environment::Var, parser::node::scope::ScopeError, ProgramState};
use anyhow::{anyhow, Result};

use super::{
    expression::Expr,
    primitive::Primitive,
    scope::{FlowStatement, Scope},
};

#[derive(Debug)]
pub enum ArgumentType {
    Ref(Var),
    Func(Func),
    Expr(Expr),
}

#[derive(Debug)]
pub struct Func {
    argument: Box<[Var]>,
    scope: Scope,
}

impl Func {
    pub fn new(argument: Box<[Var]>, scope: Scope) -> Self {
        Self { argument, scope }
    }
    pub fn execute(
        self: Rc<Self>,
        argument_input: &[ArgumentType],
        program: &ProgramState,
    ) -> Result<Primitive> {
        //only do 1 comparison here if it works
        //todo: Move it to the compilation process
        if self.argument.len() != argument_input.len() {
            if self.argument.len() < argument_input.len() {
                return Err(anyhow!("too many arguments in function call"));
            }
            return Err(anyhow!("too few arguments in function call"));
        }
        program.environment.borrow_mut().open_function_scope();
        for (i, v) in argument_input.into_iter().enumerate() {
            match v {
                ArgumentType::Func(val) => {
                    todo!()
                }
                ArgumentType::Ref(val) => {
                    let v = program.environment.borrow().get_ref(*val)?;
                    program.environment.borrow_mut().subscribe_var(*val, v)?;
                }
                ArgumentType::Expr(val) => {
                    let expr_val = val.execute(program)?;
                    program
                        .environment
                        .borrow_mut()
                        .subscribe_var(self.argument[i], expr_val)?
                }
            }
        }
        let flow_statement: Option<FlowStatement>;
        flow_statement = self.scope.execute(program, true)?;
        let rt_val = match flow_statement {
            Some(FlowStatement::Break) => Err(anyhow!(ScopeError::BreakOutsideLoop))?,
            Some(FlowStatement::Continue) => Err(anyhow!(ScopeError::ContinueOutsideLoop))?,
            Some(FlowStatement::Return(v)) => v,
            None => Primitive::VOID,
        };
        let ref_val = argument_input
            .into_iter()
            .map(|v| match v {
                ArgumentType::Ref(val) => program.environment.borrow().get_var(*val).ok(),
                _ => None,
            })
            .collect::<Vec<Option<Primitive>>>();
        program.environment.borrow_mut().close_function_scope();
        for (i, v) in ref_val.into_iter().enumerate() {
            if let Some(u) = v {
                program
                    .environment
                    .borrow_mut()
                    .assign_var(self.argument[i], u)?
            }
        }
        Ok(rt_val)
    }
}

#[derive(Debug)]

pub struct FunctionCall {
    var: Var,
    argument_input: Box<[ArgumentType]>,
}

impl FunctionCall {
    pub fn new(var: Var, argument_input: Box<[ArgumentType]>) -> Self {
        FunctionCall {
            var,
            argument_input,
        }
    }
    pub fn execute(&self, program: &ProgramState) -> Result<Primitive> {
        let func = program.environment.borrow().get_func(self.var)?;
        let val = func.execute(&self.argument_input, program)?;
        Ok(val)
    }
}
