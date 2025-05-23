use core::panic;

use crate::frontend::ast::{Parameter, Statement, TypeConstruct};

use super::evaluate::ExpressionValue;

/*
 * This file deals with creating and managing the runtime environment
 */

// Represents a function in the Wrench language, with it's closure that represents the functions in the environment at the time of declaration
#[derive(Clone)]
pub struct WrenchFunction {
    pub return_type: TypeConstruct,
    pub name: String,
    pub parameters: Vec<Parameter>,
    pub body: Box<Statement>,
    pub closure: Vec<WrenchFunction>,
}

impl WrenchFunction {
    pub fn new(
        return_type: TypeConstruct,
        name: String,
        parameters: Vec<Parameter>,
        body: Box<Statement>,
        closure: Vec<WrenchFunction>,
    ) -> Self {
        WrenchFunction {
            return_type,
            name,
            parameters,
            body,
            closure,
        }
    }

    //Convert closure to environment
    pub fn get_closure_as_env(&self) -> Vec<Vec<EnvironmentCell>> {
        let mut env = env_new();
        env_expand_scope(&mut env);
        for function in self.closure.iter() {
            env_add(&mut env, EnvironmentCell::Function(function.clone()));
        }
        env
    }
}

//Helper function to convert the environment to a closure
pub fn env_to_closure(env: &[Vec<EnvironmentCell>]) -> Vec<WrenchFunction> {
    let mut closure = Vec::new();
    for scope in env.iter() {
        for declaration in scope.iter() {
            if let EnvironmentCell::Function(function) = declaration {
                closure.push(function.clone());
            }
        }
    }
    closure
}

//Represents a cell in the environment. Only variables and functions can be defined and stored in the environment
#[derive(Clone)]
pub enum EnvironmentCell {
    Variable(String, ExpressionValue),
    Function(WrenchFunction),
}

//Helper function to retrieve a referrence to an environment cell from an environment. Returns None if the cell is not found
pub fn env_get_optional<'a>(
    env: &'a mut [Vec<EnvironmentCell>],
    name: &str,
) -> Option<&'a mut EnvironmentCell> {
    for scope in env.iter_mut().rev() {
        for declaration in scope.iter_mut() {
            match declaration {
                EnvironmentCell::Variable(var_name, _) => {
                    if var_name == name {
                        return Some(declaration);
                    }
                }
                EnvironmentCell::Function(function) => {
                    if function.name == name {
                        return Some(declaration);
                    }
                }
            }
        }
    }
    None
}

//Helper function to create a new environment
pub fn env_new() -> Vec<Vec<EnvironmentCell>> {
    Vec::new()
}

//Helper function to retrieve a referrence to an environment cell from an environment. Panics if the cell is not found
pub fn env_get(env: &[Vec<EnvironmentCell>], name: &str) -> EnvironmentCell {
    for scope in env.iter().rev() {
        for declaration in scope.iter() {
            match declaration {
                EnvironmentCell::Variable(var_name, _) => {
                    if var_name == name {
                        return declaration.clone();
                    }
                }
                EnvironmentCell::Function(function) => {
                    if function.name == name {
                        return declaration.clone();
                    }
                }
            }
        }
    }
    panic!(
        "Interpretation error. The identifier '{:?}' not found",
        name
    );
}

//Helper function to add a new environment cell to the environment. Panics if the cell is already declared
pub fn env_add(env: &mut [Vec<EnvironmentCell>], declaration: EnvironmentCell) {
    let name = match &declaration {
        EnvironmentCell::Variable(var_name, _) => var_name,
        EnvironmentCell::Function(function) => function.name.as_str(),
    };

    if env_get_optional(env, name).is_some() {
        panic!(
            "Interpretation error. The identifier '{:?}' is already declared",
            name
        );
    }

    env.last_mut().unwrap().push(declaration);
}

//Helper function to update an environment cell in the environment. Panics if the cell is not found
pub fn env_update(env: &mut [Vec<EnvironmentCell>], name: &str, expression: ExpressionValue) {
    if let Some(existing_declaration) = env_get_optional(env, name) {
        match existing_declaration {
            EnvironmentCell::Variable(_, var_expr) => {
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
