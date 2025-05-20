use std::{
    cell::RefCell,
    collections::HashMap,
    rc::Rc,
    sync::mpsc,
    thread::{self, JoinHandle},
};

use crate::frontend::ast::{Expr, Parameter, TypeConstruct};

use super::{
    environment::{env_get, EnvironmentCell, WrenchFunction},
    evaluate::{evaluate_custom_function_call, evaluate_expression, ExpressionValue},
    library::{import_csv, wrench_print},
    table::{Row, Table, TableCellType},
};

/*
 * This file deals with creating and managing pipes
 */

//Enum that represents a pipe and thereby a single thread
#[derive(Clone)]
struct SimplePipe {
    function: PipeFunction,
    args: Vec<PipeValue>,
}

impl SimplePipe {
    //Gets the table structure of how the pipe's function is called
    fn get_call_structure(&self) -> HashMap<String, TableCellType> {
        if let PipeFunction::Custom(f) = &self.function {
            let Parameter::Parameter(t, _) = f.parameters[0].clone();
            if let TypeConstruct::Table(table_type) = t {
                Table::parameters_to_structure(table_type)
            } else {
                panic!("Expected a table for the first parameter of the function");
            }
        } else {
            panic!("Expected a custom function for the pipe");
        }
    }
    //Get the table structure of how the pipe's function returns
    fn get_return_structure(&self) -> HashMap<String, TableCellType> {
        if let PipeFunction::Custom(f) = &self.function {
            if let TypeConstruct::Table(table_type) = f.return_type.clone() {
                Table::parameters_to_structure(table_type)
            } else if let TypeConstruct::Row(row_type) = f.return_type.clone() {
                return Table::parameters_to_structure(row_type);
            } else {
                panic!("Expected a table for the first parameter of the function");
            }
        } else {
            panic!("Expected a custom function for the pipe");
        }
    }
    //Determine wheter the pipe is a map, filter or reduce
    fn get_pipe_type(&self) -> PipeType {
        if let PipeFunction::Custom(f) = &self.function {
            match f.return_type {
                TypeConstruct::Table(_) => PipeType::Reduce,
                TypeConstruct::Bool => PipeType::Filter,
                _ => PipeType::Map,
            }
        } else {
            panic!("Expected a custom function for the pipe");
        }
    }
}

#[derive(Clone, Debug)]
enum PipeType {
    Map,
    Filter,
    Reduce,
}

//The value that can be passed between threads. Like expression value, tables are passed by value instead of reference
#[derive(Clone, Debug)]
pub enum PipeValue {
    Number(i32),
    Double(f64),
    String(String),
    Bool(bool),
    Table(Table),
    Row(Row),
    Array(Vec<PipeValue>),
    Null,
}

//The function that is called in the pipe. This can be a custom function or a print function
#[derive(Clone)]
enum PipeFunction {
    Print,
    Custom(WrenchFunction),
}

fn expression_value_to_pipe_value(expr: ExpressionValue) -> PipeValue {
    match expr {
        ExpressionValue::Number(n) => PipeValue::Number(n),
        ExpressionValue::Double(d) => PipeValue::Double(d),
        ExpressionValue::String(s) => PipeValue::String(s),
        ExpressionValue::Bool(b) => PipeValue::Bool(b),
        ExpressionValue::Table(t) => PipeValue::Table(t.borrow().clone()),
        ExpressionValue::Row(r) => PipeValue::Row(r),
        ExpressionValue::Array(a) => {
            PipeValue::Array(a.into_iter().map(expression_value_to_pipe_value).collect())
        }
        ExpressionValue::Null => PipeValue::Null,
    }
}

fn pipe_value_to_expression_value(expr: PipeValue) -> ExpressionValue {
    match expr {
        PipeValue::Number(n) => ExpressionValue::Number(n),
        PipeValue::Double(d) => ExpressionValue::Double(d),
        PipeValue::String(s) => ExpressionValue::String(s),
        PipeValue::Bool(b) => ExpressionValue::Bool(b),
        PipeValue::Table(t) => ExpressionValue::Table(Rc::new(RefCell::new(t))),
        PipeValue::Row(r) => ExpressionValue::Row(r),
        PipeValue::Array(a) => {
            ExpressionValue::Array(a.into_iter().map(pipe_value_to_expression_value).collect())
        }
        PipeValue::Null => ExpressionValue::Null,
    }
}

//Function that evaluates a pipe expression
pub fn evaluate_pipes(
    expr: Box<Expr>,
    function_name: String,
    args: Vec<Expr>,
    env: &mut Vec<Vec<EnvironmentCell>>,
) -> ExpressionValue {
    let (pipes, initial_expression) = pipe_rollout(expr.clone(), function_name, args, env);

    let (t1, mut rx) = init_pipe(initial_expression, env);
    let mut middle_threads = Vec::new();

    for pipe in pipes.iter() {
        let (sn, rn) = mpsc::channel();
        //let function_env = env_to_closure(&env);
        let t = pipe_middle_map(pipe.clone(), rx, sn);
        rx = rn;
        middle_threads.push(t);
    }

    let last_pipe = pipes.last().unwrap();

    let mut table;

    match &last_pipe.function {
        PipeFunction::Custom(_) => {
            // Collect the response from the last pipe into table
            table = Table::new(last_pipe.get_return_structure());
            for row in rx.iter() {
                table.add_row(row.clone());
            }
        }
        PipeFunction::Print => {
            table = Table::new(HashMap::new());
            for row in rx.iter() {
                table.add_row(row.clone());
            }
        }
    }

    // Make sure threads are finished
    t1.join().unwrap();
    for t in middle_threads {
        t.join().unwrap();
    }

    ExpressionValue::Table(Rc::new(RefCell::new(table)))
}

//Takes a pipe that can contain multiple pipes and converts them to a vector and evaluates arguments
//async_import(...) pipe x(...) pipe y(...) is converted to a vector of simple pipes and returned along with the initial expression "async_import(...)"
//Initial expression can be async_import(...) or an expression that evaluates to a table
fn pipe_rollout(
    expr: Box<Expr>,
    function_name: String,
    args: Vec<Expr>,
    env: &mut Vec<Vec<EnvironmentCell>>,
) -> (Vec<SimplePipe>, Box<Expr>) {
    let evaluated_args = args
        .iter()
        .map(|arg| expression_value_to_pipe_value(evaluate_expression(arg.clone(), env)))
        .collect::<Vec<PipeValue>>();

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
        let a_unboxed: Vec<Expr> = a.into_iter().map(|boxed| *boxed).collect();
        let (mut rest_pipes, initial_expression) = pipe_rollout(e, f, a_unboxed, env);
        rest_pipes.push(pipe);
        (rest_pipes, initial_expression)
    } else {
        //Base case
        let pipes = vec![pipe];

        (pipes, expr.clone())
    }
}

//Is responsible for evaluating the first expression of the pipe
//In async_import(...) pipe x(...), async_import(...) is evaluated in a separate thread, and values are passed to the next pipe
fn init_pipe(
    initial_expression: Box<Expr>,
    env: &mut Vec<Vec<EnvironmentCell>>,
) -> (JoinHandle<()>, mpsc::Receiver<Row>) {
    if let Expr::FunctionCall(name, args) = *initial_expression.clone() {
        if name == "async_import" {
            let left_args = args
                .iter()
                .map(|arg| expression_value_to_pipe_value(evaluate_expression(*arg.clone(), env)))
                .collect::<Vec<PipeValue>>();
            let (s, r): (mpsc::Sender<Row>, mpsc::Receiver<Row>) = mpsc::channel();
            let t = thread::spawn({
                move || {
                    pipe_import(left_args.clone(), s);
                }
            });
            (t, r)
        } else {
            let expr = evaluate_expression(*initial_expression, env);
            let (s, r): (mpsc::Sender<Row>, mpsc::Receiver<Row>) = mpsc::channel();

            if let ExpressionValue::Table(t) = expr {
                let table = t.borrow().clone();

                let t = thread::spawn({
                    move || {
                        pipe_init_table(table, s);
                    }
                });
                (t, r)
            } else {
                panic!("Table expected for the pipe");
            }
        }
    } else {
        let expr = evaluate_expression(*initial_expression, env);
        let (s, r): (mpsc::Sender<Row>, mpsc::Receiver<Row>) = mpsc::channel();

        if let ExpressionValue::Table(t) = expr {
            let table = t.borrow().clone();

            let t = thread::spawn({
                move || {
                    pipe_init_table(table, s);
                }
            });
            (t, r)
        } else {
            panic!("Table expected for the pipe");
        }
    }
}
fn pipe_middle_map(
    pipe: SimplePipe,
    receiver: mpsc::Receiver<Row>,
    sender: mpsc::Sender<Row>,
) -> JoinHandle<()> {
    match pipe.clone().function {
        PipeFunction::Custom(f) => {
            match pipe.clone().get_pipe_type() {
                PipeType::Map => {
                    // Evaluate each row at a time
                    thread::spawn({
                        move || {
                            for row in receiver {
                                let result =
                                    evaluate_fn_row_call(row.clone(), f.clone(), pipe.args.clone());
                                match result {
                                    PipeValue::Row(r) => {
                                        sender.send(r).unwrap();
                                    }
                                    _ => {
                                        panic!("Expected a row or table for the map");
                                    }
                                }
                            }
                        }
                    })
                }
                PipeType::Filter => {
                    // Evaluate each row at a time
                    thread::spawn({
                        move || {
                            for row in receiver {
                                let result =
                                    evaluate_fn_row_call(row.clone(), f.clone(), pipe.args.clone());
                                match result {
                                    PipeValue::Bool(b) => {
                                        if b {
                                            sender.send(row).unwrap();
                                        }
                                    }
                                    _ => {
                                        panic!("Expected a boolean for the filter");
                                    }
                                }
                            }
                        }
                    })
                }
                PipeType::Reduce => {
                    // Evaluate each row at a time
                    thread::spawn({
                        move || {
                            let mut table = Table::new(pipe.get_call_structure());
                            for row in receiver {
                                table.add_row(row.clone());
                            }
                            let result =
                                evaluate_fn_table_call(table, f.clone(), pipe.args.clone());
                            match result {
                                PipeValue::Table(t) => {
                                    for row in t.iter() {
                                        sender.send(row.clone()).unwrap();
                                    }
                                }
                                _ => {
                                    panic!("Expected a table for the reduce");
                                }
                            }
                        }
                    })
                }
            }
        }
        PipeFunction::Print => {
            // Evaluate each row at a time
            thread::spawn({
                move || {
                    pipe_print(receiver);
                }
            })
        }
    }
}

//Imports a CSV file one row at a time and sends it to the next pipe
fn pipe_import(args: Vec<PipeValue>, sender: mpsc::Sender<Row>) {
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
    let row_callback = move |row: Row| {
        sender.send(row).unwrap();
    };
    import_csv(name, structure, row_callback);
}

//Helper function which evaluates an entire pipe expression with posible multiple pipes to a table
fn pipe_init_table(table: Table, sender: mpsc::Sender<Row>) {
    for row in table.iter() {
        sender.send(row.clone()).unwrap();
    }
}

//Wrench library function for printing in a pipe
fn pipe_print(receiver: mpsc::Receiver<Row>) {
    // Evaluate each row at a time
    for row in receiver {
        wrench_print(vec![ExpressionValue::Row(row.clone())]);
    }
}

//Evaluates a function call where row is inserted as the first argument followed by the rest of the arguments given
fn evaluate_fn_row_call(row: Row, function: WrenchFunction, args: Vec<PipeValue>) -> PipeValue {
    let mut full_args = vec![PipeValue::Row(row)];
    full_args.extend(args);
    let expression_args: Vec<ExpressionValue> = full_args
        .iter()
        .map(|arg| pipe_value_to_expression_value(arg.clone()))
        .collect();
    let result = evaluate_custom_function_call(&function, expression_args);
    expression_value_to_pipe_value(result)
}

//Evaluates a function call where table is inserted as the first argument followed by the rest of the arguments given
fn evaluate_fn_table_call(
    table: Table,
    function: WrenchFunction,
    args: Vec<PipeValue>,
) -> PipeValue {
    let mut full_args = vec![PipeValue::Table(table)];
    full_args.extend(args);
    let expression_args: Vec<ExpressionValue> = full_args
        .iter()
        .map(|arg| pipe_value_to_expression_value(arg.clone()))
        .collect();
    let result = evaluate_custom_function_call(&function, expression_args);
    expression_value_to_pipe_value(result)
}
