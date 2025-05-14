#![allow(clippy::unused_imports)]

//use core::panic;

use std::{collections::HashMap, hash::Hash};

use csv::Reader;

use super::{environment::ExpressionValue, table::{Row, TableCell, TableCellType}};
//use csv::{Reader, StringRecord};

pub fn wrench_print(args: Vec<ExpressionValue>) -> ExpressionValue {
    for arg in args {
        match arg {
            ExpressionValue::Number(num) => println!("{}", num),
            ExpressionValue::String(s) => println!("{}", s),
            ExpressionValue::Bool(b) => println!("{}", b),
            ExpressionValue::Null => println!("Null"),
            ExpressionValue::Table(table) => {
                let table = table.borrow();
                table.print();
            }
            _ => println!("Unsupported expression type for print"),
        }
    }
    ExpressionValue::Null
}


pub fn wrench_import(args: Vec<ExpressionValue>) -> ExpressionValue{
    let file_name = match &args[0] {
        ExpressionValue::String(s) => s.clone(),
        _ => panic!("First argument must be a string"),
    };

    let mut table = match &args[1] {
        ExpressionValue::Table(table) => table.borrow_mut(),
        _ => panic!("Second argument must be a table"),
    };

    import_csv(file_name, table.get_structure().clone(), |row| {
        table.add_row(row);
    });

    ExpressionValue::Null
}



pub fn import_csv<F>(name: String, structure: HashMap<String, TableCellType>, mut row_callback: F) where F: FnMut(Row) {
    let mut reader = Reader::from_path(name).expect("Failed to open file");

    let headers = reader.headers().expect("Error reading headers").clone();
    let header_map: HashMap<&str, usize> = headers
        .iter()
        .enumerate()
        .map(|(i, name)| (name, i))
        .collect();

    for result in reader.records() {
        match result {
            Ok(record) => {
                //Parse csv record into a row
                let mut row_data: HashMap<String, TableCell> = HashMap::new();
                header_map.iter().for_each(|(name, index)| {
                    let value = record.get(*index).unwrap_or("");
                    let cell = match structure.get(*name) {
                        Some(TableCellType::Int) => TableCell::Int(value.parse::<i32>().unwrap()),
                        Some(TableCellType::String) => TableCell::String(value.to_string()),
                        Some(TableCellType::Bool) => TableCell::Bool(value.parse::<bool>().unwrap()),
                        _ => panic!("Unsupported type in table structure"),
                    };
                    row_data.insert(name.to_string(), cell);
                });
                row_callback(Row::new(row_data));
            }
            Err(e) => panic!("Error reading record: {}", e),
        }
    }
}


pub fn wrench_table_add_row(args: Vec<ExpressionValue>) -> ExpressionValue {
    let table = match &args[0] {
        ExpressionValue::Table(table) => table,
        _ => panic!("Interpretation error: Expected a table"),
    };

    let row = match &args[1] {
        ExpressionValue::Row(row) => row,
        _ => panic!("Interpretation error: Expected a row"),
    };

    table.borrow_mut().add_row(row.clone());
    ExpressionValue::Null
}