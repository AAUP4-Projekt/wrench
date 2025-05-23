use std::collections::HashMap;

use super::{
    evaluate::ExpressionValue,
    table::{Row, TableCell, TableCellType},
};
use csv::Reader;

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
#[cfg(test)]
mod tests {
    use std::{cell::RefCell, rc::Rc};

    use crate::backend::table::Table;

    use super::*;

    #[test]
    fn test_wrench_print_basic_types() {
        let args = vec![
            ExpressionValue::Number(42),
            ExpressionValue::Double(3.14),
            ExpressionValue::String("hello".to_string()),
            ExpressionValue::Bool(true),
            ExpressionValue::Null,
        ];
        // Should not panic
        let result = wrench_print(args);
        assert_eq!(result, ExpressionValue::Null);
    }

    #[test]
    fn test_wrench_print_array() {
        let arr = vec![
            ExpressionValue::Number(1),
            ExpressionValue::Number(2),
            ExpressionValue::Number(3),
        ];
        let args = vec![ExpressionValue::Array(arr)];
        let result = wrench_print(args);
        assert_eq!(result, ExpressionValue::Null);
    }

    #[test]
    #[should_panic(expected = "First argument must be a string")]
    fn test_wrench_import_invalid_first_arg() {
        let args = vec![ExpressionValue::Number(1), ExpressionValue::Null];
        wrench_import(args);
    }

    #[test]
    #[should_panic(expected = "Second argument must be a table")]
    fn test_wrench_import_invalid_second_arg() {
        let args = vec![
            ExpressionValue::String("file.csv".to_string()),
            ExpressionValue::Null,
        ];
        wrench_import(args);
    }

    #[test]
    #[should_panic(expected = "Interpretation error: Expected a table")]
    fn test_wrench_table_add_row_invalid_table() {
        let args = vec![ExpressionValue::Null, ExpressionValue::Null];
        wrench_table_add_row(args);
    }

    #[test]
    #[should_panic(expected = "Interpretation error: Expected a row")]
    fn test_wrench_table_add_row_invalid_row() {
        let mut structure = HashMap::new();
        structure.insert("id".to_string(), TableCellType::Int);
        let table = Rc::new(RefCell::new(Table::new(structure)));
        let args = vec![ExpressionValue::Table(table), ExpressionValue::Null];
        wrench_table_add_row(args);
    }
}
