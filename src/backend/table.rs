use std::collections::HashMap;

use crate::frontend::ast::{Parameter, TypeConstruct};

use super::evaluate::ExpressionValue;

/*
 * This file deals with creating and managing tables and rows
 */

#[derive(Debug, Clone, PartialEq)]
pub enum TableCell {
    Int(i32),
    Double(f64),
    String(String),
    Bool(bool),
}

#[derive(Debug, Clone, PartialEq)]
pub enum TableCellType {
    Int,
    Double,
    String,
    Bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Row {
    data: Vec<(String, TableCell)>,
}

#[derive(Debug, Clone, PartialEq)]
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
                    TableCell::Double(d) => ExpressionValue::Double(*d),
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

    pub fn get_column(&self, column_name: &str) -> ExpressionValue {
        let mut column_data = Vec::new();
        for row in &self.data {
            column_data.push(row.get(column_name));
        }
        ExpressionValue::Array(column_data)
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
#[cfg(test)]
mod tests {
    use super::*;

    fn make_structure() -> HashMap<String, TableCellType> {
        let mut s = HashMap::new();
        s.insert("id".to_string(), TableCellType::Int);
        s.insert("name".to_string(), TableCellType::String);
        s.insert("score".to_string(), TableCellType::Double);
        s.insert("active".to_string(), TableCellType::Bool);
        s
    }

    fn make_row() -> Row {
        Row::new(vec![
            ("id".to_string(), TableCell::Int(1)),
            ("name".to_string(), TableCell::String("Alice".to_string())),
            ("score".to_string(), TableCell::Double(95.5)),
            ("active".to_string(), TableCell::Bool(true)),
        ])
    }

    #[test]
    fn test_row_get() {
        let row = make_row();
        assert_eq!(row.get("id"), ExpressionValue::Number(1));
        assert_eq!(
            row.get("name"),
            ExpressionValue::String("Alice".to_string())
        );
        assert_eq!(row.get("score"), ExpressionValue::Double(95.5));
        assert_eq!(row.get("active"), ExpressionValue::Bool(true));
    }

    #[test]
    #[should_panic(expected = "Column name not found in row for missing")]
    fn test_row_get_missing_column() {
        let row = make_row();
        row.get("missing");
    }

    #[test]
    fn test_table_add_and_iter() {
        let mut table = Table::new(make_structure());
        let row1 = make_row();
        let row2 = Row::new(vec![
            ("id".to_string(), TableCell::Int(2)),
            ("name".to_string(), TableCell::String("Bob".to_string())),
            ("score".to_string(), TableCell::Double(88.0)),
            ("active".to_string(), TableCell::Bool(false)),
        ]);
        table.add_row(row1.clone());
        table.add_row(row2.clone());

        let rows: Vec<_> = table.iter().cloned().collect();
        assert_eq!(rows, vec![row1, row2]);
    }

    #[test]
    fn test_table_get_column() {
        let mut table = Table::new(make_structure());
        table.add_row(make_row());
        table.add_row(Row::new(vec![
            ("id".to_string(), TableCell::Int(2)),
            ("name".to_string(), TableCell::String("Bob".to_string())),
            ("score".to_string(), TableCell::Double(88.0)),
            ("active".to_string(), TableCell::Bool(false)),
        ]));

        let col = table.get_column("id");
        assert_eq!(
            col,
            ExpressionValue::Array(vec![ExpressionValue::Number(1), ExpressionValue::Number(2)])
        );
    }

    #[test]
    fn test_parameters_to_structure() {
        let params = vec![
            Parameter::Parameter(TypeConstruct::Int, "id".to_string()),
            Parameter::Parameter(TypeConstruct::String, "name".to_string()),
            Parameter::Parameter(TypeConstruct::Double, "score".to_string()),
            Parameter::Parameter(TypeConstruct::Bool, "active".to_string()),
        ];
        let structure = Table::parameters_to_structure(params);
        assert_eq!(structure.get("id"), Some(&TableCellType::Int));
        assert_eq!(structure.get("name"), Some(&TableCellType::String));
        assert_eq!(structure.get("score"), Some(&TableCellType::Double));
        assert_eq!(structure.get("active"), Some(&TableCellType::Bool));
    }

    #[test]
    #[should_panic(expected = "Unsupported type in table declaration for unsupported")]
    fn test_parameters_to_structure_unsupported_type() {
        let params = vec![
            Parameter::Parameter(TypeConstruct::Int, "id".to_string()),
            Parameter::Parameter(TypeConstruct::Bool, "active".to_string()),
            Parameter::Parameter(
                TypeConstruct::Array(Box::new(TypeConstruct::Int)),
                "unsupported".to_string(),
            ),
        ];
        Table::parameters_to_structure(params);
    }
}
