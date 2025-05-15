use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum TableCell {
    Int(i32),
    String(String),
    Bool(bool),
}

#[derive(Debug, Clone)]
pub enum TableCellType {
    Int,
    String,
    Bool,
}

#[derive(Debug, Clone)]
pub struct Row {
    data: HashMap<String, TableCell>,
}

#[derive(Debug, Clone)]
pub struct Table {
    data: Vec<Row>,
    structure: HashMap<String, TableCellType>,
}

impl Row {
    pub fn new(d: HashMap<String, TableCell>) -> Self {
        Row { data: d }
    }

    pub fn get(&self, column_name: &str) -> TableCell {
        match self.data.get(column_name) {
            Some(cell) => cell.clone(),
            None => panic!("Column name not found in row"),
        }
    }
}

impl Table {
    pub fn new(s: HashMap<String, TableCellType>) -> Self {
        Table {
            data: Vec::new(),
            structure: s,
        }
    }

    pub fn add_row(&mut self, row: Row) {
        self.data.push(row);
    }

    pub fn get_row(&self, index: usize) -> Row {
        match self.data.get(index) {
            Some(row) => row.clone(),
            None => panic!("Row index out of bounds"),
        }
    }

    pub fn get_column(&self, column_name: &str) -> Vec<TableCell> {
        self.data.iter().map(|row| row.get(column_name)).collect()
    }

    pub fn get_structure(&self) -> &HashMap<String, TableCellType> {
        &self.structure
    }

    pub fn print(&self) {
        for row in &self.data {
            for (key, value) in &row.data {
                match value {
                    TableCell::Int(i) => print!("{}: {}, ", key, i),
                    TableCell::String(s) => print!("{}: {}, ", key, s),
                    TableCell::Bool(b) => print!("{}: {}, ", key, b),
                }
            }
            println!();
        }
    }
}
