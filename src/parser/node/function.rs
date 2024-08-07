use std::{env, rc::Rc, thread::scope};

use crate::environment::environment::{Environment, EnvironmentItem};
use anyhow::{anyhow, Ok, Result};

use super::{expression::Expression, primitive::Primitive, scope::Scope};

#[derive(Debug)]
pub enum ArgumentType<'a> {
    Ref(&'a str),
    Func(Func<'a>),
    Expr(Expression<'a>),
}

#[derive(Debug)]
pub struct Func<'a> {
    argument: Vec<&'a str>,
    inside_environment: Environment<'a>,
    scope: Scope<'a>,
    rt_val: Option<Primitive>,
}

impl<'a> Func<'a> {
    pub fn new(argument: Vec<&'a str>, scope: Scope<'a>, rt_val: Option<Primitive>) -> Self {
        Self {
            argument,
            inside_environment: Environment::default(),
            scope,
            rt_val,
        }
    }
    pub fn execute(
        &'a self,
        argument_input: &'a Vec<ArgumentType<'a>>,
        environment: &Environment<'a>,
    ) -> Result<Environment<'a>> {
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
                    let expr_val = val.evaluation(&environment)?;
                    func_environment = func_environment
                        .subscribe(self.argument[i], EnvironmentItem::Primitive(expr_val))?;
                }
            }
        }
        self.scope.execute(func_environment)
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
    pub fn execute(&'a self, environment: Environment<'a>) -> Result<Environment> {
        let func = match environment.get_var(&self.identifier)? {
            EnvironmentItem::Func(v) => v,
            _ => Err(anyhow!("{} is not a function", self.identifier))?,
        };
        func.execute(&self.argument_input, &environment)?;
        Ok(environment)
    }
}
