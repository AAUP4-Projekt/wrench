use std::{collections::HashMap, sync::mpsc};

use crate::frontend::ast::Expr;

use super::{
    environment::{EnvironmentCell, ExpressionValue, env_get},
    evaluate::evaluate_expression,
    library::import_csv,
    table::{Row, Table, TableCellType},
};

struct SimplePipe {
    function_name: String,
    args: Vec<ExpressionValue>,
}

pub fn evaluate_pipes(
    expr: Box<Expr>,
    function_name: String,
    args: Vec<Box<Expr>>,
    env: &mut Vec<Vec<EnvironmentCell>>,
) -> ExpressionValue {
    let pipes = pipe_rollout(expr, function_name, args, env);

    return ExpressionValue::Null;
}

fn pipe_rollout(
    expr: Box<Expr>,
    function_name: String,
    args: Vec<Box<Expr>>,
    env: &mut Vec<Vec<EnvironmentCell>>,
) -> Vec<SimplePipe> {
    let mut result: Vec<SimplePipe> = Vec::new();

    for (arg) in args.iter() {
        let evaluated_args = args
            .iter()
            .map(|arg| evaluate_expression(*arg.clone(), env))
            .collect::<Vec<ExpressionValue>>();
        let function_name = match *arg.clone() {
            Expr::FunctionCall(name, _) => name.clone(),
            _ => panic!("Expected function call"),
        };

        let pipe = SimplePipe {
            function_name,
            args: evaluated_args,
        };

        result.push(pipe);
    }
    return result;
}

//Imports a CSV file one row at a time and sends it to the next pipe
fn pipe_import(
    name: String,
    structure: HashMap<String, TableCellType>,
    sender: mpsc::Sender<ExpressionValue>,
) {
    let row_callback = |row: Row| {
        sender.send(ExpressionValue::Row(row)).unwrap();
    };
    import_csv(name, structure, row_callback);
}

/*
fn pipe_middle(pipe: SimplePipe, function_env: Vec<Vec<EnvironmentCell>>, receiver: mpsc::Receiver<ExpressionValue>, sender: mpsc::Sender<ExpressionValue>) {
    let function_name = pipe.function_name;
    let function = env_get(&function_env, &function_name);

}*/
