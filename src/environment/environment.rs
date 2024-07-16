extern crate fxhash;

use anyhow::{anyhow, Result};

use fxhash::FxHashMap;

use crate::parser::node::primitive::Primitive;

#[derive(Default)]
pub struct Environment<'a> {
    scope_depth: u16,
    variable_mp: FxHashMap<&'a str, Vec<(Box<dyn Primitive>, u16)>>,
}

impl<'a> Environment<'a> {
    pub fn subscribe(mut self, identifier: &'a str, value: Box<dyn Primitive>) -> Result<Self> {
        if !self.variable_mp.contains_key(identifier) {
            let val = vec![(value, self.scope_depth)];
            self.variable_mp.insert(identifier, val);
            return Ok(self);
        }
        let val = self.variable_mp.get_mut(identifier).unwrap();
        let (_, depth) = val.last().unwrap();
        if *depth == self.scope_depth {
            return Err(anyhow!(
                "The variable {} has already declared in the current scope",
                identifier
            ));
        }

        val.push((value, self.scope_depth));

        Ok(self)
    }
    pub fn get_var(&self, identifier: &'a str) -> Result<&Box<dyn Primitive>> {
        match self.variable_mp.get(identifier).and_then(|val| val.last()) {
            Some((val, _)) => Ok(val),
            None => Err(anyhow!("{} hasn't been declared", identifier))?,
        }
    }
}
