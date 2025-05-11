use core::panic;
use std::collections::HashMap;

use crate::frontend::ast::{Declaration, Expr, Operator, Parameter, Statement, TypeConstruct};

use super::{
    environment::{
        EnvironmentCell, ExpressionValue, env_add, env_expand_scope, env_get, env_new, env_update,
    },
    library::wrench_print,
    table::{Table, TableCellType},
};

const UNIMPLEMENTED_ERROR: &str =
    "Interpretation error: Unimplemented evaluation for abstract syntax tree node";

pub fn interpret(input: Statement) {
    let mut env = env_new();
    env_expand_scope(&mut env);
    evaluate_statement(Box::new(input), &mut env);
}

//Evaluate single statement
fn evaluate_statement(statement: Box<Statement>, env: &mut Vec<Vec<EnvironmentCell>>) {
    match *statement {
        Statement::Declaration(declaration) => {
            evaluate_declaration(declaration, env);
        }
        Statement::Expr(expression) => {
            evaluate_expression(*expression, env);
        }
        Statement::VariableAssignment(variable, expression) => {
            let evaluated_value = evaluate_expression(*expression, env);
            env_update(env, &variable, evaluated_value);
        }
        Statement::Compound(s1, s2) => {
            evaluate_statement(s1, env);
            evaluate_statement(s2, env);
        }
        Statement::Skip => {}
        _ => {
            panic!("{}", UNIMPLEMENTED_ERROR);
        }
    }
}

fn evaluate_declaration(declaration: Declaration, env: &mut Vec<Vec<EnvironmentCell>>) {
    match declaration {
        Declaration::Variable(var_type, var_name, value) => {
            let evaluated_value = evaluate_expression(*value, env);
            env_add(
                env,
                EnvironmentCell::Variable(var_type, var_name, evaluated_value),
            );
        }
        Declaration::Function(func_type, func_name, parameters, body) => {
            env_add(
                env,
                EnvironmentCell::Function(func_type, func_name, parameters, body, env.clone()),
            );
        }
        _ => {
            panic!("{}", UNIMPLEMENTED_ERROR);
        }
    }
}

fn evaluate_expression(expression: Expr, env: &mut Vec<Vec<EnvironmentCell>>) -> ExpressionValue {
    match expression {
        Expr::Number(n) => ExpressionValue::Number(n),
        Expr::Bool(b) => ExpressionValue::Bool(b),
        Expr::StringLiteral(s) => ExpressionValue::String(s),
        Expr::Operation(e1, op, e2) => {
            let left = evaluate_expression(*e1, env);
            let right = evaluate_expression(*e2, env);
            evaluate_operation(left, op, right)
        }
        Expr::Identifier(ref name) => match env_get(env, &name) {
            EnvironmentCell::Variable(_, _, value) => value,
            EnvironmentCell::Function(..) => {
                panic!("Interpretation error: Function identifier not allowed as expression")
            }
        },
        Expr::FunctionCall(name, expressions) => {
            let mut args: Vec<ExpressionValue> = Vec::new();
            for expression in expressions {
                args.push(evaluate_expression(*expression, env));
            }
            evaluate_function_call(name, args, env)
        }
        Expr::Table(params) => {
            let mut structure: HashMap<String, TableCellType> = HashMap::new();
            for param in params {
                match param {
                    Parameter::Parameter(t, name) => match t {
                        TypeConstruct::Bool => {
                            structure.insert(name.clone(), TableCellType::Bool);
                        }
                        TypeConstruct::Int => {
                            structure.insert(name.clone(), TableCellType::Int);
                        }
                        TypeConstruct::String => {
                            structure.insert(name.clone(), TableCellType::String);
                        }
                        _ => {
                            panic!("Interpretation error: Unsupported type in table declaration")
                        }
                    },
                }
            }
            ExpressionValue::Table(Table::new(structure))
        }
        _ => {
            panic!("{}", UNIMPLEMENTED_ERROR);
        }
    }
}

fn evaluate_function_call(
    name: String,
    args: Vec<ExpressionValue>,
    env: &mut Vec<Vec<EnvironmentCell>>,
) -> ExpressionValue {
    match name.as_str() {
        "print" => wrench_print(args),
        _ => {
            let function = env_get(env, &name);
            if let EnvironmentCell::Function(_, _, _, statement, mut closure) = function {
                evaluate_statement(statement, &mut closure);
            } else {
                panic!(
                    "Interpretation error: Identifier '{:?}' is not a function",
                    name
                );
            }
            ExpressionValue::Null
        }
    }
}

fn evaluate_operation(
    left: ExpressionValue,
    operator: Operator,
    right: ExpressionValue,
) -> ExpressionValue {
    match operator {
        Operator::Addition => {
            if let (ExpressionValue::Number(l), ExpressionValue::Number(r)) = (&left, &right) {
                return ExpressionValue::Number(l + r);
            } else if let (ExpressionValue::String(l), ExpressionValue::String(r)) = (&left, &right)
            {
                return ExpressionValue::String(format!("{}{}", l, r));
            }
        }
        Operator::Or => {
            if let (ExpressionValue::Bool(l), ExpressionValue::Bool(r)) = (left, right) {
                return ExpressionValue::Bool(l || r);
            }
        }
        Operator::Equals => {
            if let (ExpressionValue::Bool(l), ExpressionValue::Bool(r)) = (&left, &right) {
                return ExpressionValue::Bool(l == r);
            } else if let (ExpressionValue::Number(l), ExpressionValue::Number(r)) = (&left, &right)
            {
                return ExpressionValue::Bool(l == r);
            } else if let (ExpressionValue::String(l), ExpressionValue::String(r)) = (&left, &right)
            {
                return ExpressionValue::Bool(l == r);
            }
        }
        _ => {}
    }
    panic!("Interpretation error: Unsupported operation")
}
