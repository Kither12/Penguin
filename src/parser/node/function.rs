use crate::environment::environment::{Environment, EnvironmentItem};
use anyhow::{anyhow, Ok, Result};

use super::{
    expression::Expression,
    primitive::Primitive,
    scope::{FlowStatement, Scope, ScopeError},
};

#[derive(Debug)]
pub enum ArgumentType<'a> {
    Ref(&'a str),
    Func(Func<'a>),
    Expr(Expression<'a>),
}

#[derive(Debug)]
pub struct Func<'a> {
    argument: Vec<&'a str>,
    scope: Scope<'a>,
}

impl<'a> Func<'a> {
    pub fn new(argument: Vec<&'a str>, scope: Scope<'a>) -> Self {
        Self { argument, scope }
    }
    pub fn execute(
        &'a self,
        argument_input: &'a Vec<ArgumentType<'a>>,
        environment: &Environment<'a>,
    ) -> Result<(Environment<'a>, Primitive)> {
        //only do 1 comparison here if it works
        //todo: Move it to the compilation process
        if self.argument.len() != argument_input.len() {
            if self.argument.len() < argument_input.len() {
                return Err(anyhow!("too many arguments in function call"));
            }
            return Err(anyhow!("too few arguments in function call"));
        }

        let mut func_environment = environment.clone();
        func_environment = func_environment.open_scope();
        for (i, v) in argument_input.into_iter().enumerate() {
            match v {
                ArgumentType::Func(val) => {
                    todo!()
                }
                ArgumentType::Ref(_) => {}
                ArgumentType::Expr(val) => {
                    let (env, expr_val) = val.execute(func_environment)?;
                    func_environment =
                        env.subscribe(self.argument[i], EnvironmentItem::Primitive(expr_val))?;
                }
            }
        }
        let flow_statement: Option<FlowStatement>;

        (func_environment, flow_statement) = self.scope.execute(func_environment)?;
        match flow_statement {
            Some(FlowStatement::Break) => Err(anyhow!(ScopeError::BreakOutsideLoop))?,
            Some(FlowStatement::Continue) => Err(anyhow!(ScopeError::ContinueOutsideLoop))?,
            _ => {}
        };
        Ok((func_environment, Primitive::void()))
    }
}

#[derive(Debug)]

pub struct FunctionCall<'a> {
    identifier: &'a str,
    argument_input: Vec<ArgumentType<'a>>,
}

impl<'a> FunctionCall<'a> {
    pub fn new(identifier: &'a str, argument_input: Vec<ArgumentType<'a>>) -> Self {
        FunctionCall {
            identifier,
            argument_input,
        }
    }
    pub fn execute(&'a self, environment: Environment<'a>) -> Result<(Environment<'a>, Primitive)> {
        let v = environment.get_var(&self.identifier)?;
        let func = match v.1.as_ref() {
            EnvironmentItem::Func(v) => v,
            _ => Err(anyhow!("{} is not a function", self.identifier))?,
        };
        let (_, val) = func.execute(&self.argument_input, &v.0)?;
        Ok((v.0, val))
    }
}
