extern crate fxhash;

use std::{
    cell::{Cell, RefCell},
    rc::Rc,
};

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

#[derive(Debug, Default)]
pub struct Environment<'a> {
    scope_depth: Cell<usize>,
    scope_stack: RefCell<Vec<(&'a str, usize)>>,
    variable_mp: RefCell<FxHashMap<&'a str, Vec<(Rc<Primitive>, usize)>>>,
    function_mp: RefCell<FxHashMap<&'a str, Vec<(Rc<Func<'a>>, usize)>>>,
}

impl<'a> Environment<'a> {
    pub fn can_declare(&self, identifier: &'a str) -> bool {
        if let Some(var_stack) = self.function_mp.borrow().get(identifier) {
            if let Some((_, depth)) = var_stack.last() {
                if *depth == self.scope_depth.get() {
                    return false;
                }
            }
        }
        if let Some(var_stack) = self.variable_mp.borrow().get(identifier) {
            if let Some((_, depth)) = var_stack.last() {
                if *depth == self.scope_depth.get() {
                    return false;
                }
            }
        }
        return true;
    }
    pub fn subscribe_func(&self, identifier: &'a str, value: Rc<Func<'a>>) -> Result<()> {
        if self.can_declare(identifier) == false {
            return Err(anyhow!(EnvironmentError::ReDeclaration(
                identifier.to_owned()
            )));
        }
        if let Some(var_stack) = self.function_mp.borrow_mut().get_mut(identifier) {
            var_stack.push((value, self.scope_depth.get()));
            self.scope_stack
                .borrow_mut()
                .push((identifier, self.scope_depth.get()));
            return Ok(());
        }
        let mut val = Vec::with_capacity(64);
        val.push((value, self.scope_depth.get()));
        self.function_mp.borrow_mut().insert(identifier, val);
        self.scope_stack
            .borrow_mut()
            .push((identifier, self.scope_depth.get()));
        Ok(())
    }

    pub fn subscribe_var(&self, identifier: &'a str, value: Rc<Primitive>) -> Result<()> {
        if self.can_declare(identifier) == false {
            return Err(anyhow!(EnvironmentError::ReDeclaration(
                identifier.to_owned()
            )));
        }
        if let Some(var_stack) = self.variable_mp.borrow_mut().get_mut(identifier) {
            var_stack.push((value, self.scope_depth.get()));
            self.scope_stack
                .borrow_mut()
                .push((identifier, self.scope_depth.get()));
            return Ok(());
        }
        let mut val = Vec::with_capacity(64);
        val.push((value, self.scope_depth.get()));
        self.variable_mp.borrow_mut().insert(identifier, val);
        self.scope_stack
            .borrow_mut()
            .push((identifier, self.scope_depth.get()));
        Ok(())
    }
    pub fn get_var(&'a self, identifier: &'a str) -> Result<Rc<Primitive>> {
        let x = self
            .variable_mp
            .borrow()
            .get(identifier)
            .and_then(|val| val.last())
            .map(|(v, _)| v.clone())
            .ok_or_else(|| anyhow!(EnvironmentError::NotDeclareation(identifier.to_owned())))?;
        Ok(x)
    }
    pub fn get_func(&'a self, identifier: &'a str) -> Result<Rc<Func<'a>>> {
        let x = self
            .function_mp
            .borrow()
            .get(identifier)
            .and_then(|val| val.last())
            .map(|(v, _)| v.clone())
            .ok_or_else(|| anyhow!(EnvironmentError::NotDeclareation(identifier.to_owned())))?;
        Ok(x)
    }
    pub fn assign_var(&self, identifier: &'a str, value: Rc<Primitive>) -> Result<()> {
        *self
            .variable_mp
            .borrow_mut()
            .get_mut(identifier)
            .and_then(|val| val.last_mut())
            .map(|(v, _)| v)
            .ok_or_else(|| anyhow!(EnvironmentError::NotDeclareation(identifier.to_owned())))? =
            value;
        Ok(())
    }
    pub fn open_scope(&self) {
        self.scope_depth.set(self.scope_depth.get() + 1);
    }
    pub fn close_scope(&self) {
        let mut st = self.scope_stack.borrow_mut();
        while let Some((key, depth)) = st.last() {
            if *depth == self.scope_depth.get() {
                self.variable_mp
                    .borrow_mut()
                    .get_mut(key)
                    .and_then(|v| v.pop());
                st.pop();
            } else {
                break;
            }
        }
        self.scope_depth.set(self.scope_depth.get() - 1);
    }
}
