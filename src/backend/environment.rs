use core::panic;

use crate::frontend::ast::{Expr, Parameter, Statement, TypeConstruct};
use std::rc::Rc;
use std::cell::RefCell;

use super::table::{Row, Table};

#[derive(Clone)]
pub enum ExpressionValue {
    Number(i32),
    String(String),
    Bool(bool),
    Table(Rc<RefCell<Table>>),
    Row(Row),
    Array(Vec<ExpressionValue>),
    Null,
}

pub enum StatementValue{
    None,
    Return(ExpressionValue),
}

#[derive(Clone)]
pub enum EnvironmentCell {
    Variable(TypeConstruct, String, ExpressionValue),
    Function(
        TypeConstruct,
        String,
        Vec<Parameter>,
        Box<Statement>,
        Vec<Vec<EnvironmentCell>>,
    ),
}

fn env_get_optional<'a>(
    env: &'a mut Vec<Vec<EnvironmentCell>>,
    name: &str,
) -> Option<&'a mut EnvironmentCell> {
    for scope in env.iter_mut().rev() {
        for declaration in scope.iter_mut() {
            match declaration {
                EnvironmentCell::Variable(_, var_name, _) => {
                    if var_name == name {
                        return Some(declaration);
                    }
                }
                EnvironmentCell::Function(_, func_name, _, _, _) => {
                    if func_name == name {
                        return Some(declaration);
                    }
                }
            }
        }
    }
    None
}

pub fn env_new() -> Vec<Vec<EnvironmentCell>> {
    Vec::new()
}

pub fn env_get(env: &mut Vec<Vec<EnvironmentCell>>, name: &str) -> EnvironmentCell {
    if let Some(value) = env_get_optional(env, name) {
        return value.clone();
    }
    panic!(
        "Interpretation error. The identifier '{:?}' not found",
        name
    );
}

pub fn env_add(env: &mut Vec<Vec<EnvironmentCell>>, declaration: EnvironmentCell) {
    let name = match &declaration {
        EnvironmentCell::Variable(_, var_name, _) => var_name,
        EnvironmentCell::Function(_, func_name, _, _, _) => func_name,
    };

    if env_get_optional(env, name).is_some() {
        panic!(
            "Interpretation error. The identifier '{:?}' is already declared",
            name
        );
    }

    env.last_mut().unwrap().push(declaration);
}

pub fn env_update(env: &mut Vec<Vec<EnvironmentCell>>, name: &str, expression: ExpressionValue) {
    if let Some(existing_declaration) = env_get_optional(env, name) {
        match existing_declaration {
            EnvironmentCell::Variable(_, _, var_expr) => {
                *var_expr = expression;
            }
            _ => {
                panic!("Interpretation error. Only variables can be reassgined");
            }
        }
        return;
    }
    panic!(
        "Interpretation error. The identifier '{:?}' not found in the environment",
        name
    );
}

pub fn env_expand_scope(env: &mut Vec<Vec<EnvironmentCell>>) {
    env.push(Vec::new());
}

pub fn env_shrink_scope(env: &mut Vec<Vec<EnvironmentCell>>) {
    env.pop();
}

pub fn env_clone_functions(env: &mut Vec<Vec<EnvironmentCell>>) -> Vec<Vec<EnvironmentCell>> {
    let mut new_env = env_new();
    env_expand_scope(&mut new_env);

    for scope in env.iter() {
        for declaration in scope.iter() {
            match declaration {
                EnvironmentCell::Function(..) => {
                    env_add(&mut new_env, declaration.clone());
                }
                _ => {}
            }
        }
    }

    new_env
}