use crate::frontend::ast::{Expr, Statement, Declaration};

use super::{environment::{env_get, env_update}, library::wrench_print};



pub fn interpret(input: Vec<Statement>){
    let mut env: Vec<Vec<Declaration>> = Vec::new(); // Initialize the environment stack
    env.push(Vec::new()); // Push the global scope

    for statement in input {
        evaluate_statement(statement, &mut env);
    }
}

fn evaluate_statement(statement: Statement, env: &mut Vec<Vec<Declaration>>){
    match statement {
        Statement::Declaration(declaration) => {
            evaluate_declaration(declaration, env);
        }
        Statement::Expr(expression) => {
            evaluate_expression(*expression, env);
        }
        Statement::VariableAssignment(variable, expression, ) => {
            let evaluated_value = evaluate_expression(*expression, env);
            env_update(env, &variable, Box::new(evaluated_value));
        }
        _ => {}
    }
}

fn evaluate_declaration(declaration: Declaration, env: &mut Vec<Vec<Declaration>>){
    match declaration {
        Declaration::Variable(var_type, var_name, value) => {
            let evaluated_value = evaluate_expression(*value, env);
            env.last_mut().unwrap().push(Declaration::Variable(var_type, var_name, Box::new(evaluated_value)));
        }
        _ => {}
    }
}

fn evaluate_expression(expression: Expr, env: &mut Vec<Vec<Declaration>>) -> Expr {
    match expression {
        Expr::Number(_) => expression,
        Expr::FunctionCall(name, args) => {
            let evaluated_args: Vec<Expr> = args.into_iter().map(|arg| evaluate_expression(*arg, env)).collect();
            evaluate_function_call(name.to_string(), evaluated_args)
        },
        Expr::Identifier(ref name) => {
            match env_get(env, &name) {
                Declaration::Variable(_, _, value) => *value,
                Declaration::Constant(_, _, value) => *value,
                Declaration::Function(_, _, _, _) => expression.clone(),
            }
        },
        _ => panic!("Unsupported expression type {:?}", expression),
    }
}


fn evaluate_function_call(name: String, args: Vec<Expr>) -> Expr {
    match name.as_str(){
        "print" => wrench_print(args),
        _ => panic!("Function {} not found", name),
    }
}