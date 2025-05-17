use std::collections::HashMap;

use crate::frontend::ast::{Parameter, TypeConstruct};

use super::environment::ExpressionValue;

#[derive(Debug, Clone)]
pub enum TableCell {
    Int(i32),
    Double(f64),
    String(String),
    Bool(bool),
}

#[derive(Debug, Clone)]
pub enum TableCellType {
    Int,
    Double,
    String,
    Bool,
}

#[derive(Debug, Clone)]
pub struct Row {
    data: Vec<(String, TableCell)>,
}

#[derive(Debug, Clone)]
pub struct Table {
    data: Vec<Row>,
    structure: HashMap<String, TableCellType>,
}

impl Row {
    pub fn new(d: Vec<(String, TableCell)>) -> Self {
        Row { data: d }
    }

    pub fn get(&self, column_name: &str) -> ExpressionValue {
        for (key, value) in &self.data {
            if key == column_name {
                return match value {
                    TableCell::Int(i) => ExpressionValue::Number(*i),
                    TableCell::Double(d) => ExpressionValue::Number(*d as i32),
                    TableCell::String(s) => ExpressionValue::String(s.clone()),
                    TableCell::Bool(b) => ExpressionValue::Bool(*b),
                };
            }
        }
        panic!("Column name not found in row for {}", column_name);
    }

    pub fn print(&self) {
        for (key, value) in &self.data {
            match value {
                TableCell::Int(i) => print!("{}: {}, ", key, i),
                TableCell::Double(d) => print!("{}: {}, ", key, d),
                TableCell::String(s) => print!("{}: {}, ", key, s),
                TableCell::Bool(b) => print!("{}: {}, ", key, b),
            }
        }
        println!();
    }
}

impl Table {
    pub fn new(s: HashMap<String, TableCellType>) -> Self {
        Table {
            data: Vec::new(),
            structure: s,
        }
    }
    pub fn iter(&self) -> impl Iterator<Item = &Row> {
        self.data.iter()
    }

    pub fn add_row(&mut self, row: Row) {
        self.data.push(row);
    }

    pub fn get_structure(&self) -> &HashMap<String, TableCellType> {
        &self.structure
    }

    pub fn parameters_to_structure(parameters: Vec<Parameter>) -> HashMap<String, TableCellType> {
        let mut structure = HashMap::new();
        for param in parameters {
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
                    TypeConstruct::Double => {
                        structure.insert(name.clone(), TableCellType::Double);
                    }
                    _ => {
                        panic!("Unsupported type in table declaration for {}", name);
                    }
                },
            }
        }
        structure
    }

    pub fn print(&self) {
        for row in &self.data {
            row.print();
        }
    }
}
