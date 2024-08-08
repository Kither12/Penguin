extern crate fxhash;

use std::rc::Rc;

use anyhow::{anyhow, Result};

use fxhash::FxHashMap;

use crate::parser::node::{function::Func, primitive::Primitive};

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
#[derive(Debug, Clone)]
pub enum EnvironmentItem<'a> {
    Primitive(Primitive),
    Func(Rc<Func<'a>>),
}

#[derive(Debug, Default)]
pub struct Environment<'a> {
    scope_depth: usize,
    scope_stack: Vec<(&'a str, usize)>,
    variable_mp: FxHashMap<&'a str, Vec<(Rc<EnvironmentItem<'a>>, usize)>>,
}

impl<'a> Clone for Environment<'a> {
    fn clone(&self) -> Self {
        // When cloning, we only care about the value in the latest scope
        let mut variable_mp = FxHashMap::default();
        for (k, v) in &self.variable_mp {
            if let Some((identifier, _)) = v.last() {
                let mut val = Vec::with_capacity(64);
                val.push((identifier.clone(), 0usize));
                variable_mp.insert(*k, val);
            }
        }

        Self {
            scope_depth: 0,
            scope_stack: Vec::default(),
            variable_mp: variable_mp,
        }
    }
}

impl<'a> Environment<'a> {
    pub fn subscribe(mut self, identifier: &'a str, value: EnvironmentItem<'a>) -> Result<Self> {
        if let Some(var_stack) = self.variable_mp.get_mut(identifier) {
            if let Some((_, depth)) = var_stack.last() {
                if *depth == self.scope_depth {
                    return Err(anyhow!(EnvironmentError::ReDeclaration(
                        identifier.to_owned()
                    )));
                }
            }
            var_stack.push((Rc::new(value), self.scope_depth));
        } else {
            let mut val = Vec::with_capacity(64);
            val.push((Rc::new(value), self.scope_depth));
            self.variable_mp.insert(identifier, val);
        }
        self.scope_stack.push((identifier, self.scope_depth));
        Ok(self)
    }
    pub fn get_var(self, identifier: &'a str) -> Result<(Self, Rc<EnvironmentItem>)> {
        let x = self
            .variable_mp
            .get(identifier)
            .and_then(|val| val.last())
            .map(|(v, _)| Rc::clone(v))
            .ok_or_else(|| anyhow!(EnvironmentError::NotDeclareation(identifier.to_owned())))?;
        Ok((self, x))
    }
    pub fn assign_var(mut self, identifier: &'a str, value: EnvironmentItem<'a>) -> Result<Self> {
        *self
            .variable_mp
            .get_mut(identifier)
            .and_then(|val| val.last_mut())
            .map(|(v, _)| v)
            .ok_or_else(|| anyhow!(EnvironmentError::NotDeclareation(identifier.to_owned())))? =
            Rc::new(value);
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
