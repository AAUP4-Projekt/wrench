use std::{collections::HashMap, sync::mpsc};

use crate::frontend::ast::Expr;

use super::{environment::{EnvironmentCell, ExpressionValue}, library::import_csv, table::{Row, Table, TableCellType}};



pub fn evaluate_pipes(expr: Box<Expr>, function_name: String, args: Vec<Box<Expr>>, env: &mut Vec<Vec<EnvironmentCell>>) -> ExpressionValue {

    

    return ExpressionValue::Null;
}

//Imports a CSV file one row at a time and sends it to the next pipe
fn pipe_import(name: String, structure: HashMap<String, TableCellType>, sender: mpsc::Sender<ExpressionValue>){
    let row_callback = |row: Row| {
        sender.send(ExpressionValue::Row(row)).unwrap();
    };
    import_csv(name, structure, row_callback);
}