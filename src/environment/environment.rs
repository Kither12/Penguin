extern crate fxhash;

use anyhow::{anyhow, Result};

use fxhash::FxHashMap;

use crate::parser::node::primitive::Primitive;

#[derive(Default)]
pub struct Environment<'a> {
    scope_depth: u16,
    scope_stack: Vec<(&'a str, u16)>,
    variable_mp: FxHashMap<&'a str, Vec<(Box<dyn Primitive>, u16)>>,
}

impl<'a> Environment<'a> {
    pub fn subscribe(mut self, identifier: &'a str, value: Box<dyn Primitive>) -> Result<Self> {
        if !self.variable_mp.contains_key(identifier) {
            let val = vec![(value, self.scope_depth)];
            self.variable_mp.insert(identifier, val);
            self.scope_stack.push((identifier, self.scope_depth));
            return Ok(self);
        }
        let var_stack = self.variable_mp.get_mut(identifier).unwrap();
        let (_, depth) = var_stack.last().unwrap();
        if *depth == self.scope_depth {
            return Err(anyhow!(
                "{} has already been declared in the current scope",
                identifier
            ));
        }

        var_stack.push((value, self.scope_depth));
        self.scope_stack.push((identifier, self.scope_depth));
        Ok(self)
    }
    pub fn get_var(&self, identifier: &'a str) -> Result<&Box<dyn Primitive>> {
        match self.variable_mp.get(identifier).and_then(|val| val.last()) {
            Some((val, _)) => Ok(val),
            None => Err(anyhow!("{} hasn't been declared", identifier)),
        }
    }
    pub fn assign_var(mut self, identifier: &'a str, value: Box<dyn Primitive>) -> Result<Self> {
        match self
            .variable_mp
            .get_mut(identifier)
            .and_then(|val| val.last_mut())
        {
            Some((val, _)) => {
                *val = value;
                Ok(self)
            }
            None => Err(anyhow!("{} hasn't been declared", identifier)),
        }
    }
    pub fn open_scope(mut self) -> Self {
        self.scope_depth += 1;
        self
    }
    pub fn close_scope(mut self) -> Self {
        while let Some((key, depth)) = self.scope_stack.last() {
            if *depth == self.scope_depth {
                let var_stack = self.variable_mp.get_mut(key).unwrap();
                var_stack.pop();
                self.scope_stack.pop();
            } else {
                break;
            }
        }
        self.scope_depth -= 1;
        self
    }
}
