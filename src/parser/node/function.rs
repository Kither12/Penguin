use std::rc::Rc;

use crate::environment::environment::Environment;
use anyhow::{anyhow, Ok, Result};

use super::{
    expression::ExpressionPool,
    primitive::Primitive,
    scope::{FlowStatement, Scope, ScopeError},
};

#[derive(Debug)]
pub enum ArgumentType<'a> {
    Ref(&'a str),
    Func(Func<'a>),
    Expr(ExpressionPool<'a>),
}

#[derive(Debug)]
pub struct Func<'a> {
    argument: Box<[&'a str]>,
    scope: Scope<'a>,
}

impl<'a> Func<'a> {
    pub fn new(argument: Box<[&'a str]>, scope: Scope<'a>) -> Self {
        Self { argument, scope }
    }
    pub fn execute(
        self: Rc<Self>,
        argument_input: &'a [ArgumentType<'a>],
        environment: &'a Environment<'a>,
    ) -> Result<Primitive> {
        //only do 1 comparison here if it works
        //todo: Move it to the compilation process
        if self.argument.len() != argument_input.len() {
            if self.argument.len() < argument_input.len() {
                return Err(anyhow!("too many arguments in function call"));
            }
            return Err(anyhow!("too few arguments in function call"));
        }

        let func_environment = Environment::default();
        func_environment.open_scope();
        for (i, v) in argument_input.into_iter().enumerate() {
            match v {
                ArgumentType::Func(val) => {
                    todo!()
                }
                ArgumentType::Ref(val) => {
                    func_environment.subscribe_var(val, environment.get_var(val)?)?
                }
                ArgumentType::Expr(val) => {
                    let expr_val = val.execute(&environment)?;
                    func_environment.subscribe_var(self.argument[i], Rc::new(expr_val))?
                }
            }
        }
        let flow_statement: Option<FlowStatement>;
        flow_statement = self.scope.execute(&func_environment, true)?;
        let rt_val = match flow_statement {
            Some(FlowStatement::Break) => Err(anyhow!(ScopeError::BreakOutsideLoop))?,
            Some(FlowStatement::Continue) => Err(anyhow!(ScopeError::ContinueOutsideLoop))?,
            Some(FlowStatement::Return(v)) => v,
            None => Primitive::VOID,
        };
        // for (i, v) in argument_input.into_iter().enumerate() {
        //     match v {
        //         ArgumentType::Ref(val) => environment
        //             .assign_var(self.argument[i], func_environment.get_var(val)?.clone())?,
        //         _ => {}
        //     }
        // }

        Ok(rt_val)
    }
}

#[derive(Debug)]

pub struct FunctionCall<'a> {
    identifier: &'a str,
    argument_input: Box<[ArgumentType<'a>]>,
}

impl<'a> FunctionCall<'a> {
    pub fn new(identifier: &'a str, argument_input: Box<[ArgumentType<'a>]>) -> Self {
        FunctionCall {
            identifier,
            argument_input,
        }
    }
    pub fn execute(&'a self, environment: &'a Environment<'a>) -> Result<Primitive> {
        let func = environment.get_func(&self.identifier)?;
        let val = func.execute(&self.argument_input, environment)?;
        Ok(val)
    }
}
