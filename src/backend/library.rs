#![allow(clippy::unused_imports)]

//use core::panic;

use super::environment::ExpressionValue;
//use csv::{Reader, StringRecord};

pub fn wrench_print(args: Vec<ExpressionValue>) -> ExpressionValue {
    for arg in args {
        match arg {
            ExpressionValue::Number(num) => println!("{}", num),
            ExpressionValue::String(s) => println!("{}", s),
            ExpressionValue::Bool(b) => println!("{}", b),
            _ => println!("Unsupported expression type for print"),
        }
    }
    ExpressionValue::Null
}

/*
pub fn wrench_import(args: Vec<ExpressionValue>) -> ExpressionValue{
    let table = &args[1];
    if let ExpressionValue::String(file_name) = &args[0] {
        let mut reader = Reader::from_path(file_name).expect("Failed to open file");
        for result in reader.records() {
            match result {
                Ok(record) => {
                    record.iter().for_each(|field| {
                        println!("{}", field);
                    });
                }
                Err(e) => println!("Error reading record: {}", e),
            }
        }
        panic!("Interpretation error: Expected a string for file name")
    } else {
        panic!("Interpretation error: Expected a string for file name")
    }
}
*/
