use core::panic;
use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::frontend::ast::{
    ColumnAssignmentEnum, Declaration, Expr, Operator, Parameter, Statement, TypeConstruct,
};

use super::{
    environment::{
        EnvironmentCell, ExpressionValue, StatementValue, env_add, env_expand_scope, env_get,
        env_new, env_shrink_scope, env_update,
    },
    library::{wrench_import, wrench_print, wrench_table_add_row},
    pipes::evaluate_pipes,
    table::{Row, Table, TableCell, TableCellType},
};

const UNIMPLEMENTED_ERROR: &str =
    "Interpretation error: Unimplemented evaluation for abstract syntax tree node";

pub fn interpret(input: Statement) {
    let mut env = env_new();
    env_expand_scope(&mut env);
    evaluate_statement(Box::new(input), &mut env);
}

//Evaluate single statement
fn evaluate_statement(
    statement: Box<Statement>,
    env: &mut Vec<Vec<EnvironmentCell>>,
) -> StatementValue {
    match *statement {
        Statement::Declaration(declaration) => {
            evaluate_declaration(declaration, env);
            StatementValue::None
        }
        Statement::Expr(expression) => {
            evaluate_expression(*expression, env);
            StatementValue::None
        }
        Statement::VariableAssignment(variable, expression) => {
            let evaluated_value = evaluate_expression(*expression, env);
            env_update(env, &variable, evaluated_value);
            StatementValue::None
        }
        Statement::Compound(s1, s2) => {
            let s1v = evaluate_statement(s1, env);

            match s1v {
                StatementValue::Return(_) => {
                    return s1v;
                }
                _ => {}
            }

            let s2v: StatementValue = evaluate_statement(s2, env);

            match s2v {
                StatementValue::Return(_) => {
                    return s1v;
                }
                StatementValue::None => {
                    return StatementValue::None;
                }
            }
        }
        Statement::Skip => StatementValue::None,
        Statement::Return(expression) => {
            let return_value = evaluate_expression(*expression, env);
            env_shrink_scope(env);
            return StatementValue::Return(return_value);
        }
        _ => {
            panic!("{}", UNIMPLEMENTED_ERROR);
        }
        Statement::For(parameter, expression, body) => {
            // for x in evaluate_expression(expression, env) {
            //     evaluate_statement(body, env);
            // }

            // for int x in [1,2,3]{
            //     print(x);
            // }

            // for int x in [2,3]

            // for int x in [3]

            // DONE

            // let array = evaluate_expression(expression, env);
            // if array != [] {
            //     evaluate_statement(body, env);
            //     array.pop

            // }
            StatementValue::None
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

pub fn evaluate_expression(
    expression: Expr,
    env: &mut Vec<Vec<EnvironmentCell>>,
) -> ExpressionValue {
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
        Expr::Row(column_assignment) => {
            let mut row: HashMap<String, TableCell> = HashMap::new();
            for assignment in column_assignment {
                match assignment {
                    ColumnAssignmentEnum::ColumnAssignment(_, name, expression) => {
                        let evaluated_value = evaluate_expression(*expression, env);
                        match evaluated_value {
                            ExpressionValue::Number(n) => {
                                row.insert(name.clone(), TableCell::Int(n));
                            }
                            ExpressionValue::String(s) => {
                                row.insert(name.clone(), TableCell::String(s));
                            }
                            ExpressionValue::Bool(b) => {
                                row.insert(name.clone(), TableCell::Bool(b));
                            }
                            _ => {
                                panic!("Interpretation error: Unsupported type in row assignment")
                            }
                        }
                    }
                }
            }
            ExpressionValue::Row(Row::new(row))
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
            ExpressionValue::Table(Rc::new(RefCell::new(Table::new(structure))))
        }
        Expr::Pipe(expression, function_name, args) => {
            //evaluate_pipes(expression, function_name, args, env)
            ExpressionValue::Null
        }
        // Expr::Array(args) => {
        //     ExpressionValue::Array((args))
        // }
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
        "import" => wrench_import(args),
        "table_add_row" => wrench_table_add_row(args),
        _ => {
            let function = env_get(env, &name);
            if let EnvironmentCell::Function(_, _, _, statement, mut closure) = function {
                let statement_value = evaluate_statement(statement, &mut closure);
                match statement_value {
                    StatementValue::Return(value) => value,
                    StatementValue::None => ExpressionValue::Null,
                }
            } else {
                panic!(
                    "Interpretation error: Identifier '{:?}' is not a function",
                    name
                );
            }
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
