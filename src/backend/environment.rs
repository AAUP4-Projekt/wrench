use crate::frontend::ast::{Declaration, Expr};

fn env_get_optional<'a>(env: &'a mut Vec<Vec<Declaration>>, name: &str) -> Option<&'a mut Declaration> {
    for scope in env.iter_mut().rev() {
        for declaration in scope.iter_mut() {
            match declaration {
                Declaration::Variable(_, var_name, _) if name == *var_name => {
                    return Some(declaration);
                }
                Declaration::Constant(_, const_name, _) if name == *const_name => {
                    return Some(declaration);
                }
                Declaration::Function(_, func_name, _, _) if name == *func_name => {
                    return Some(declaration);
                }
                _ => {}
            }
        }
    }
    None
}

pub fn env_get(env: &mut Vec<Vec<Declaration>>, name: &str) -> Declaration {
    if let Some(value) = env_get_optional(env, name) {
        return value.clone();
    }
    panic!("Interpretation error. The identifier '{:?}' not found in the environment", name);
}

pub fn env_add(env: &mut Vec<Vec<Declaration>>, declaration: Declaration) {
    let name = match &declaration {
        Declaration::Variable(_, var_name, _) => var_name,
        Declaration::Constant(_, const_name, _) => const_name,
        Declaration::Function(_, func_name, _, _) => func_name,
    };
    if env_get_optional(env, name).is_some() {
        panic!("Interpretation error. The identifier '{:?}' is already declared in the current scope", name);
    }
    env.last_mut().unwrap().push(declaration);
}

pub fn env_update(env: &mut Vec<Vec<Declaration>>, name: &str, expression: Box<Expr>) {
    if let Some(existing_declaration) = env_get_optional(env, name) {
        match existing_declaration {
            Declaration::Variable(_, _, var_expr) => {
                *var_expr = expression;
            }
            _ => {
                panic!("Interpretation error. Only variables can be reassgined");
            }
        }
        return;
    }
    panic!("Interpretation error. The identifier '{:?}' not found in the environment", name);
}

pub fn env_expand_scope(env: &mut Vec<Vec<Declaration>>) {
    env.push(Vec::new());
}

pub fn env_shrink_scope(env: &mut Vec<Vec<Declaration>>) {
    if env.len() > 1 {
        env.pop();
    } else {
        panic!("Interpretation error. Cannot shrink the global scope");
    }
}