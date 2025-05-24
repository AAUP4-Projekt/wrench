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
#[cfg(test)]
mod tests {
    use super::*;

    fn dummy_function(name: &str) -> WrenchFunction {
        WrenchFunction::new(
            TypeConstruct::Int,
            name.to_string(),
            vec![],
            Box::new(Statement::Skip),
            vec![],
        )
    }

    fn dummy_variable(name: &str, value: i32) -> EnvironmentCell {
        EnvironmentCell::Variable(name.to_string(), ExpressionValue::Number(value))
    }

    #[test]
    fn test_env_new_and_expand_shrink_scope() {
        let mut env = env_new();
        assert_eq!(env.len(), 0);
        env_expand_scope(&mut env);
        assert_eq!(env.len(), 1);
        env_shrink_scope(&mut env);
        assert_eq!(env.len(), 0);
    }

    #[test]
    fn test_env_add_and_get_variable() {
        let mut env = env_new();
        env_expand_scope(&mut env);
        env_add(&mut env, dummy_variable("x", 42));
        let cell = env_get(&env, "x");
        match cell {
            EnvironmentCell::Variable(ref name, ExpressionValue::Number(val)) => {
                assert_eq!(name, "x");
                assert_eq!(val, 42);
            }
            _ => self::panic!("Expected variable"),
        }
    }

    #[test]
    #[should_panic]
    fn test_env_add_duplicate_panics() {
        let mut env = env_new();
        env_expand_scope(&mut env);
        env_add(&mut env, dummy_variable("x", 1));
        env_add(&mut env, dummy_variable("x", 2)); // Should panic
    }

    #[test]
    fn test_env_add_and_get_function() {
        let mut env = env_new();
        env_expand_scope(&mut env);
        let func = dummy_function("foo");
        env_add(&mut env, EnvironmentCell::Function(func.clone()));
        let cell = env_get(&env, "foo");
        match cell {
            EnvironmentCell::Function(f) => {
                assert_eq!(f.name, "foo");
            }
            _ => self::panic!("Expected function"),
        }
    }

    #[test]
    fn test_env_update_variable() {
        let mut env = env_new();
        env_expand_scope(&mut env);
        env_add(&mut env, dummy_variable("x", 10));
        env_update(&mut env, "x", ExpressionValue::Number(99));
        let cell = env_get(&env, "x");
        match cell {
            EnvironmentCell::Variable(_, ExpressionValue::Number(val)) => assert_eq!(val, 99),
            _ => self::panic!("Expected variable"),
        }
    }

    #[test]
    #[should_panic]
    fn test_env_update_nonexistent_panics() {
        let mut env = env_new();
        env_expand_scope(&mut env);
        env_update(&mut env, "y", ExpressionValue::Number(1)); // Should panic
    }

    #[test]
    #[should_panic]
    fn test_env_update_function_panics() {
        let mut env = env_new();
        env_expand_scope(&mut env);
        let func = dummy_function("foo");
        env_add(&mut env, EnvironmentCell::Function(func));
        env_update(&mut env, "foo", ExpressionValue::Number(1)); // Should panic
    }

    #[test]
    fn test_env_get_optional() {
        let mut env = env_new();
        env_expand_scope(&mut env);
        env_add(&mut env, dummy_variable("x", 5));
        assert!(env_get_optional(&mut env, "x").is_some());
        assert!(env_get_optional(&mut env, "y").is_none());
    }

    #[test]
    fn test_env_to_closure_and_get_closure_as_env() {
        let func1 = dummy_function("f1");
        let func2 = dummy_function("f2");
        let closure = vec![func1.clone(), func2.clone()];
        let wrench_func = WrenchFunction::new(
            TypeConstruct::Int,
            "main".to_string(),
            vec![],
            Box::new(Statement::Skip),
            closure.clone(),
        );
        let env = wrench_func.get_closure_as_env();
        let closure_from_env = env_to_closure(&env);
        assert_eq!(closure_from_env.len(), 2);
        assert!(closure_from_env.iter().any(|f| f.name == "f1"));
        assert!(closure_from_env.iter().any(|f| f.name == "f2"));
    }
}
