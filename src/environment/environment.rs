extern crate fxhash;

use anyhow::{anyhow, Result};

use fxhash::FxHashMap;

use crate::parser::node::primitive::Primitive;

#[derive(Debug)]
enum EnvironmentError {
    ReDeclaration(String),
    NotDeclareation(String),
}

impl std::fmt::Display for EnvironmentError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ReDeclaration(i) => write!(f, "{i} has been previously declared"),
            Self::NotDeclareation(i) => write!(f, "{i} was not declared"),
        }
    }
}

#[derive(Default)]
pub struct Environment<'a> {
    scope_depth: usize,
    scope_stack: Vec<(&'a str, usize)>,
    variable_mp: FxHashMap<&'a str, Vec<(Primitive, usize)>>,
}

impl<'a> Environment<'a> {
    pub fn subscribe(mut self, identifier: &'a str, value: Primitive) -> Result<Self> {
        if let Some(var_stack) = self.variable_mp.get_mut(identifier) {
            if let Some((_, depth)) = var_stack.last() {
                if *depth == self.scope_depth {
                    return Err(anyhow!(EnvironmentError::ReDeclaration(
                        identifier.to_owned()
                    )));
                }
            }
            var_stack.push((value, self.scope_depth));
        } else {
            let mut val = Vec::with_capacity(64);
            val.push((value, self.scope_depth));
            self.variable_mp.insert(identifier, val);
        }
        self.scope_stack.push((identifier, self.scope_depth));
        Ok(self)
    }
    pub fn get_var(&self, identifier: &'a str) -> Result<&Primitive> {
        self.variable_mp
            .get(identifier)
            .and_then(|val| val.last())
            .map(|(v, _)| v)
            .ok_or_else(|| anyhow!(EnvironmentError::NotDeclareation(identifier.to_owned())))
    }
    pub fn assign_var(mut self, identifier: &'a str, value: Primitive) -> Result<Self> {
        *self
            .variable_mp
            .get_mut(identifier)
            .and_then(|val| val.last_mut())
            .map(|(v, _)| v)
            .ok_or_else(|| anyhow!(EnvironmentError::NotDeclareation(identifier.to_owned())))? =
            value;
        Ok(self)
    }
    pub fn open_scope(mut self) -> Self {
        self.scope_depth += 1;
        self
    }
    pub fn close_scope(mut self) -> Self {
        while let Some((key, depth)) = self.scope_stack.last() {
            if *depth == self.scope_depth {
                self.variable_mp.get_mut(key).and_then(|v| v.pop());
                self.scope_stack.pop();
            } else {
                break;
            }
        }
        self.scope_depth -= 1;
        self
    }
}
