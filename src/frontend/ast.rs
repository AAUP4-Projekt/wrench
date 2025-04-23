use std::fmt::Debug;

#[derive(PartialEq, Debug)]
pub enum Statement {
    Expr(Box<Expr>), // Represents an expression statement
    VariableDeclaration(TypeConstruct, String, Box<Expr>), // Represents a variable declaration with its type, name, and assigned value
    VariableAssignment(String, Box<Expr>), // Represents a variable assignment with its name and value
}

#[derive(PartialEq, Debug)]
pub enum Expr {
    Number(i32), // Represents a number
    Identifier(String), // Represents an identifier (variable name)
    Bool(bool), // Represents a boolean value
    Operation(Box<Expr>, Operator, Box<Expr>), // Represents an operation with left and right operands and an operator
}

// Enum representing types
#[derive(PartialEq, Debug)]
pub enum TypeConstruct {
    Bool,
    Int,
    Double,
    String
}

// Enum representing the different types of operations
#[derive(PartialEq, Debug)]
pub enum Operator {
    Mul, // multiplication (*)
    Div, // division (/)
    Add, // addition (+)
    Sub, // subtraction (-)
}

