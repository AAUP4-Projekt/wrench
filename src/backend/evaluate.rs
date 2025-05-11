use core::panic;

use crate::frontend::ast::{Declaration, Expr, Operator, Statement};

use super::{environment::{env_add, env_expand_scope, env_get, env_new, env_update, EnvironmentCell}, library::wrench_print};

const UNIMPLEMENTED_ERROR: &str = "Interpretation error: Unimplemented evaluation for abstract syntax tree node";

pub fn interpret(input: Statement){
    let mut env = env_new();
    env_expand_scope(&mut env);
    evaluate_statement(Box::new(input), &mut env);
}

//Evaluate single statement
fn evaluate_statement(statement: Box<Statement>, env: &mut Vec<Vec<EnvironmentCell>>){
    match *statement {
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
        Statement::Compound(s1, s2) => {
            evaluate_statement(s1, env);
            evaluate_statement(s2, env);
        }
        Statement::Skip => {},
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
        Expr::Bool(_) => expression,
        Expr::StringLiteral(_) => expression,
        Expr::Operation(e1,op , e2) => {
            let left = evaluate_expression(*e1, env);
            let right = evaluate_expression(*e2, env);
            evaluate_operation(left, op, right)
        },
        Expr::Identifier(ref name) => {
            match env_get(env, &name) {
                EnvironmentCell::Variable(_, _, value) => value,
                EnvironmentCell::Function(..) => panic!("Interpretation error: Function identifier not allowed as expression"),
            }
        },        
        Expr::FunctionCall(name, args) => {
            let evaluated_args: Vec<Expr> = args.into_iter().map(|arg| evaluate_expression(*arg, env)).collect();
            evaluate_function_call(name.to_string(), evaluated_args, env)
        },
        _ => {panic!("{}", UNIMPLEMENTED_ERROR);}
    }
}


fn evaluate_function_call(name: String, args: Vec<Expr>, env: &mut Vec<Vec<EnvironmentCell>>) -> Expr {
    match name.as_str(){
        "print" => wrench_print(args),
        _ => {

            let function = env_get(env, &name);
            if let EnvironmentCell::Function(_, _, _, statement, mut closure) = function {
                evaluate_statement(statement, &mut closure);
            } else {
                panic!("Interpretation error: Identifier '{:?}' is not a function", name);
            }
            Expr::Null
        }
    }
}

fn evaluate_operation(left: Expr, operator: Operator, right: Expr) -> Expr {
    match operator {
        Operator::Addition => {
            if let (Expr::Number(l), Expr::Number(r)) = (&left, &right) {
                return Expr::Number(l + r);
            } else if let (Expr::Double(l), Expr::Double(r)) = (&left, &right) {
                return Expr::Double(l + r);
            } else if let (Expr::StringLiteral(l), Expr::StringLiteral(r)) = (&left, &right) {
                return Expr::StringLiteral(format!("{}{}", l, r));
            }
        }
        Operator::Or => {
            if let (Expr::Bool(l), Expr::Bool(r)) = (left, right) {
                return Expr::Bool(l || r);
            }
        }
        Operator::Equals => {
            if let (Expr::Bool(l), Expr::Bool(r)) = (&left, &right) {
                return Expr::Bool(l == r);
            } else if let (Expr::Number(l), Expr::Number(r)) = (&left, &right) {
                return Expr::Bool(l == r);
            } else if let (Expr::StringLiteral(l), Expr::StringLiteral(r)) = (&left, &right) {
                return Expr::Bool(l == r);
            }
        }
        _ => {}
    }
    panic!("Interpretation error: Unsupported operation")
}