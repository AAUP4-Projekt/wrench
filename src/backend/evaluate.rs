use core::panic;

use crate::frontend::ast::{Expr, Statement, Declaration};

use super::{environment::{env_add, env_expand_scope, env_get, env_new, env_update, EnvironmentCell}, library::wrench_print};

const UNIMPLEMENTED_ERROR: &str = "Interpretation error: Unimplemented evaluation for abstract syntax tree node";

pub fn interpret(input: Vec<Statement>){
    let mut env = env_new();
    env_expand_scope(&mut env);
    evaluate_statements(input, &mut env);
}

//Evaluate multiplie statements sequentially
fn evaluate_statements(statements: Vec<Statement>, env: &mut Vec<Vec<EnvironmentCell>>) {
    for statement in statements {
        evaluate_statement(statement, env);
    }
}

//Evaluate single statement
fn evaluate_statement(statement: Statement, env: &mut Vec<Vec<EnvironmentCell>>){
    match statement {
        Statement::Declaration(declaration) => {
            evaluate_declaration(declaration, env);
        }
        Statement::Expr(expression) => {
            evaluate_expression(*expression, env);
        }
        Statement::VariableAssignment(variable, expression, ) => {
            let evaluated_value = evaluate_expression(*expression, env);
            env_update(env, &variable, evaluated_value);
        }
        _ => {panic!("{}", UNIMPLEMENTED_ERROR);}
    }
}

fn evaluate_declaration(declaration: Declaration, env: &mut Vec<Vec<EnvironmentCell>>){
    match declaration {
        Declaration::Variable(var_type, var_name, value) => {
            let evaluated_value = evaluate_expression(*value, env);
            env_add(env, EnvironmentCell::Variable(var_type, var_name, evaluated_value));
        }
        Declaration::Function(func_type, func_name, parameters, body) => {
            env_add(env, EnvironmentCell::Function(func_type, func_name, parameters, body, env.clone()));
        }
        _ => {panic!("{}", UNIMPLEMENTED_ERROR);}
    }
}

fn evaluate_expression(expression: Expr, env: &mut Vec<Vec<EnvironmentCell>>) -> Expr {
    match expression {
        Expr::Number(_) => expression,
        Expr::FunctionCall(name, args) => {
            let evaluated_args: Vec<Expr> = args.into_iter().map(|arg| evaluate_expression(*arg, env)).collect();
            evaluate_function_call(name.to_string(), evaluated_args, env)
        },
        Expr::Identifier(ref name) => {
            match env_get(env, &name) {
                EnvironmentCell::Variable(_, _, value) => value,
                EnvironmentCell::Function(..) => panic!("Interpretation error: Function identifier not allowed as expression"),
            }
        },
        _ => {panic!("{}", UNIMPLEMENTED_ERROR);}
    }
}


fn evaluate_function_call(name: String, args: Vec<Expr>, env: &mut Vec<Vec<EnvironmentCell>>) -> Expr {
    match name.as_str(){
        "print" => wrench_print(args),
        _ => {

            let function = env_get(env, &name);
            if let EnvironmentCell::Function(_, _, _, statements, mut closure) = function {
                evaluate_statements(statements, &mut closure);
            } else {
                panic!("Interpretation error: Identifier '{:?}' is not a function", name);
            }
            Expr::Null
        }
    }
}