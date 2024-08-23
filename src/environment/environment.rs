extern crate fxhash;

use std::{f32::consts::E, rc::Rc};

use anyhow::{anyhow, Result};

use fxhash::FxHashMap;

use crate::parser::node::{function::Func, primitive::Primitive};

#[derive(Debug)]
enum EnvironmentError {
    ReDeclaration,
    NotDeclareation,
}

impl std::fmt::Display for EnvironmentError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ReDeclaration => write!(f, "variable has been previously declared"),
            Self::NotDeclareation => write!(f, "variable was not declared"),
        }
    }
}

#[derive(Debug, Default)]
pub struct Environment<'a> {
    scope_depth: usize,
    function_scope: Vec<usize>,
    scope_stack: Vec<(Var, usize)>,
    var_mp: FxHashMap<&'a str, Var>,
    variable_mp: Vec<Vec<(Primitive, usize)>>,
    function_mp: Vec<Vec<(Rc<Func>, usize)>>,
}
#[derive(Debug, Clone, Copy)]
pub struct Var(usize);

impl<'a> Environment<'a> {
    pub fn register(&mut self, var: &'a str) -> Var {
        match self.var_mp.get(var) {
            Some(v) => *v,
            None => {
                self.var_mp.insert(var, Var(self.var_mp.len()));
                Var(self.var_mp.len() - 1)
            }
        }
    }
    pub fn init(&mut self) {
        self.function_scope.push(0);
        self.variable_mp
            .resize(self.var_mp.len(), Default::default());
        self.function_mp
            .resize(self.var_mp.len(), Default::default());
    }
    pub fn check_declare(&self, var: Var) -> Result<()> {
        if let Some((_, depth)) = self.variable_mp[var.0].last() {
            if *depth == self.scope_depth {
                return Err(anyhow!(EnvironmentError::ReDeclaration));
            }
        }
        if let Some((_, depth)) = self.function_mp[var.0].last() {
            if *depth == self.scope_depth {
                return Err(anyhow!(EnvironmentError::ReDeclaration));
            }
        }
        return Ok(());
    }
    pub fn subscribe_func(&mut self, var: Var, value: Rc<Func>) -> Result<()> {
        self.check_declare(var)?;
        self.function_mp[var.0].push((value, self.scope_depth));
        self.scope_stack.push((var, self.scope_depth));
        return Ok(());
    }

    pub fn subscribe_var(&mut self, var: Var, value: Primitive) -> Result<()> {
        self.check_declare(var)?;
        self.variable_mp[var.0].push((value, self.scope_depth));
        self.scope_stack.push((var, self.scope_depth));
        return Ok(());
    }
    pub fn get_var(&'a self, var: Var) -> Result<Primitive> {
        self.variable_mp[var.0]
            .last()
            .and_then(|(v, depth)| {
                if depth < self.function_scope.last().unwrap() {
                    None
                } else {
                    Some(*v)
                }
            })
            .ok_or_else(|| anyhow!(EnvironmentError::NotDeclareation))
    }
    pub fn get_ref(&'a self, var: Var) -> Result<Primitive> {
        self.variable_mp[var.0]
            .last()
            .map(|(v, _)| *v)
            .ok_or_else(|| anyhow!(EnvironmentError::NotDeclareation))
    }
    pub fn get_func(&self, var: Var) -> Result<Rc<Func>> {
        self.function_mp[var.0]
            .last()
            .and_then(|(v, depth)| {
                if depth < self.function_scope.last().unwrap() {
                    None
                } else {
                    Some(v.clone())
                }
            })
            .ok_or_else(|| anyhow!(EnvironmentError::NotDeclareation))
    }
    pub fn assign_var(&mut self, var: Var, value: Primitive) -> Result<()> {
        *self.variable_mp[var.0]
            .last_mut()
            .map(|(v, _)| v)
            .ok_or_else(|| anyhow!(EnvironmentError::NotDeclareation))? = value;
        Ok(())
    }
    pub fn open_scope(&mut self) {
        self.scope_depth = self.scope_depth + 1;
    }
    pub fn close_scope(&mut self) {
        while let Some((var, depth)) = self.scope_stack.last() {
            if *depth == self.scope_depth {
                self.variable_mp[var.0].pop();
                self.scope_stack.pop();
            } else {
                break;
            }
        }
        self.scope_depth = self.scope_depth - 1;
    }
    pub fn open_function_scope(&mut self) {
        self.open_scope();
        self.function_scope.push(self.scope_depth);
    }
    pub fn close_function_scope(&mut self) {
        self.close_scope();
        self.function_scope.pop();
    }
}
