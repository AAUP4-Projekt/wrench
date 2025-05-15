use std::{cell::RefCell, collections::HashMap, rc::Rc, sync::mpsc, thread};

use crate::frontend::ast::{Expr, Parameter, Statement, TypeConstruct};

use super::{environment::{env_get, EnvironmentCell, ExpressionValue, WrenchFunction}, evaluate::{evaluate_expression, evaluate_function_call}, library::import_csv, table::{Row, Table, TableCellType}};

#[derive(Clone)]
struct SimplePipe {
    function: PipeFunction,
    args: Vec<PipeValue>,
}

impl SimplePipe{
    fn get_structure(&self) -> HashMap<String, TableCellType> {
        if let PipeFunction::Custom(f) = &self.function {
            let Parameter::Parameter(t, s) = f.parameters[0].clone();
            if let TypeConstruct::Table(table_type) = t {
                return Table::parameters_to_structure(table_type);
            } else {
                panic!("Expected a table for the first parameter of the function");
            }
        } else {
            panic!("Expected a custom function for the pipe");
        }
    }
}


#[derive(Clone, Debug)]
pub enum PipeValue {
    Number(i32),
    String(String),
    Bool(bool),
    Table(Table),
    Row(Row),
    Array(Vec<PipeValue>),
    Null,
}

#[derive(Clone)]
enum PipeFunction{
    Print,
    Custom(WrenchFunction),
}

fn expression_value_to_pipe_value(expr: ExpressionValue) -> PipeValue {
    match expr {
        ExpressionValue::Number(n) => PipeValue::Number(n),
        ExpressionValue::String(s) => PipeValue::String(s),
        ExpressionValue::Bool(b) => PipeValue::Bool(b),
        ExpressionValue::Table(t) => PipeValue::Table(t.borrow().clone()),
        ExpressionValue::Row(r) => PipeValue::Row(r),
        ExpressionValue::Array(a) => PipeValue::Array(a.into_iter().map(expression_value_to_pipe_value).collect()),
        ExpressionValue::Null => PipeValue::Null,
    }
}

fn pipe_value_to_expression_value(expr: PipeValue) -> ExpressionValue {
    match expr {
        PipeValue::Number(n) => ExpressionValue::Number(n),
        PipeValue::String(s) => ExpressionValue::String(s),
        PipeValue::Bool(b) => ExpressionValue::Bool(b),
        PipeValue::Table(t) => ExpressionValue::Table(Rc::new(RefCell::new(t))),
        PipeValue::Row(r) => ExpressionValue::Row(r),
        PipeValue::Array(a) => ExpressionValue::Array(a.into_iter().map(pipe_value_to_expression_value).collect()),
        PipeValue::Null => ExpressionValue::Null,
    }
}



pub fn evaluate_pipes(expr: Box<Expr>, function_name: String, args: Vec<Box<Expr>>, env: &mut Vec<Vec<EnvironmentCell>>) -> ExpressionValue {

    let (pipes, initial_expression) = pipe_rollout(expr.clone(), function_name, args, env);

    let (x_1, y_1): (mpsc::Sender<PipeValue>, mpsc::Receiver<PipeValue>) = mpsc::channel();

    let left_args = if let Expr::FunctionCall(_, args) = *initial_expression {
        args.iter().map(|arg| expression_value_to_pipe_value(evaluate_expression(*arg.clone(), env))).collect::<Vec<PipeValue>>()
    } else {
        panic!("Expected a function call for the leftmost side of the pipes");
    };

    let t_i = thread::spawn(move || {
            pipe_import(left_args.clone(), x_1);
    });

    let t_e = thread::spawn(move || {
            pipe_print(y_1);
    });


    t_i.join().unwrap();
    t_e.join().unwrap();

    /*

    let import_structure = if let ExpressionValue::Table(t) = pipes[0].args[1].clone() {
        t.borrow().get_structure().clone()
    } else {
        panic!("Expected a table for the pipe structure");
    };


    let (x_1, y_1): (mpsc::Sender<PipeValue>, mpsc::Receiver<PipeValue>) = mpsc::channel();
    */

    /*
    let import_thread = thread::spawn(move || {
            pipe_test(x_1);
    });
    */
    

    return ExpressionValue::Null;
}

fn pipe_rollout(expr: Box<Expr>, function_name: String, args: Vec<Box<Expr>>, env: &mut Vec<Vec<EnvironmentCell>>) -> (Vec<SimplePipe>, Box<Expr>) {
    let evaluated_args = args.iter().map(|arg| expression_value_to_pipe_value( evaluate_expression(*arg.clone(), env))).collect::<Vec<PipeValue>>();


    let function = match function_name.as_str() {
        "print" => PipeFunction::Print,
        _ => {
            if let EnvironmentCell::Function(f) = env_get(env, &function_name) {
                PipeFunction::Custom(f)
            } else {
                panic!("Expected a function for the pipe");
            }
        }
    };

    let pipe = SimplePipe {
        function: function.clone(),
        args: evaluated_args,
    };

    // Collect through recursion
    if let Expr::Pipe(e, f, a) = *expr {
        let (mut rest_pipes, initial_expression) = pipe_rollout(e, f, a, env);
        rest_pipes.push(pipe);
        return (rest_pipes, initial_expression);
    } else {
        //Base case
        let mut pipes = Vec::new();
        pipes.push(pipe);

        return (pipes, expr.clone());
    }
}

/*
fn pipe_print(pipes: Vec<SimplePipe>){
    for pipe in pipes {
        let function_name = pipe.function_name;
        println!("Pipe function: {}", function_name);
    }
}
*/

//Imports a CSV file one row at a time and sends it to the next pipe
fn pipe_import(args: Vec<PipeValue>, sender: mpsc::Sender<PipeValue>){
    let name = if let PipeValue::String(s) = args[0].clone() {
        s
    } else {
        panic!("Expected a string literal for the first argument of pipe_import");
    };
    let structure = if let PipeValue::Table(t) = args[1].clone() {
        t.get_structure().clone()
    } else {
        panic!("Expected a table for the second argument of pipe_import");
    };
    let row_callback = |row: Row| {
        sender.send(PipeValue::Row(row)).unwrap();
    };
    import_csv(name, structure, row_callback);
}

fn pipe_print(receiver: mpsc::Receiver<PipeValue>) {
    // Evaluate each row at a time
    for row in receiver {
        match row {
            PipeValue::Row(r) => {
                println!("{:?}", r);
            }
            _ => {
                panic!("Expected a row for the pipe_print");
            }
        }
    }
}

/*
fn pipe_middle(pipe: SimplePipe, function_env: &Vec<WrenchFunction>, receiver: mpsc::Receiver<PipeValue>, sender: mpsc::Sender<PipeValue>) {
    let mut function_env = function_env;

    // Evaluate each row at a time
    for row in receiver {
        let result = evaluate_fn_row_call(row, pipe.function_name.clone(), pipe.args.clone(), &mut function_env);
        sender.send(result).unwrap();
    }
}
*/
/* 
fn pipe_end(pipe: SimplePipe, function_env: &Vec<WrenchFunction>, receiver: mpsc::Receiver<PipeValue>) {
    let mut function_env = function_env;
    // Evaluate each row at a time
    for row in receiver {
        let result = evaluate_fn_row_call(row, pipe.function_name.clone(), pipe.args.clone(), &mut function_env);
        print!("{:?}", result);
    }
}
*/

fn evaluate_fn_row_call(row : PipeValue, function: WrenchFunction, args: Vec<PipeValue>) -> PipeValue {
    let mut full_args = vec![row];
    full_args.extend(args);
    let expression_args: Vec<ExpressionValue> = full_args.iter().map(|arg| pipe_value_to_expression_value(arg.clone())).collect();
    let result = evaluate_function_call(function.name.clone(), expression_args, &function.get_closure_as_env());
    expression_value_to_pipe_value(result)
}