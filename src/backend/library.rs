use std::collections::HashMap;

use csv::Reader;

use super::{
    environment::ExpressionValue,
    table::{Row, TableCell, TableCellType},
};

/*
 * This file contains the wrench library functions, and helper functions for those
 */

// Wrench function for printing it's actual parameters. Returns null
pub fn wrench_print(args: Vec<ExpressionValue>) -> ExpressionValue {
    for arg in args {
        match arg {
            ExpressionValue::Number(num) => println!("{}", num),
            ExpressionValue::Double(num) => println!("{}", num),
            ExpressionValue::String(s) => println!("{}", s),
            ExpressionValue::Bool(b) => println!("{}", b),
            ExpressionValue::Null => println!("Null"),
            ExpressionValue::Row(row) => {
                row.print();
            }
            ExpressionValue::Table(table) => {
                let table = table.borrow();
                table.print();
            }
            ExpressionValue::Array(arr) => {
                for item in arr {
                    wrench_print(vec![item]);
                }
            }
        }
    }
    ExpressionValue::Null
}

// Wrench library function for importing a table from a CSV file. Called with a file name and a table which types and columns matches a csv file
pub fn wrench_import(args: Vec<ExpressionValue>) -> ExpressionValue {
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

    args[1].clone()
}

// Helper function to Itterate over a CSV file and call the callback function for each row
pub fn import_csv<F>(name: String, structure: HashMap<String, TableCellType>, mut row_callback: F)
where
    F: FnMut(Row),
{
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
                let mut row_data: Vec<(String, TableCell)> = Vec::new();
                for (name, cell_type) in &structure {
                    if let Some(index) = header_map.get(name.as_str()) {
                        let value = record.get(*index).unwrap_or("");
                        let cell = match cell_type {
                            TableCellType::Int => TableCell::Int(value.parse::<i32>().unwrap()),
                            TableCellType::String => TableCell::String(value.to_string()),
                            TableCellType::Bool => TableCell::Bool(value.parse::<bool>().unwrap()),
                            TableCellType::Double => {
                                TableCell::Double(value.parse::<f64>().unwrap())
                            }
                        };
                        row_data.push((name.clone(), cell));
                    } else {
                        panic!("CSV file is missing column '{}'", name);
                    }
                }
                row_callback(Row::new(row_data));
            }
            Err(e) => panic!("Error reading record: {}", e),
        }
    }
}

// Wrench library function for adding a row to a table. Called with a table and a row
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
