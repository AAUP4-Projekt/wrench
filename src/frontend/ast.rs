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
    Op(Box<Expr>, Opcode, Box<Expr>), // Represents an operation with left and right operands and an operator
}

// Enum representing types
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum TypeConstruct {
    Bool,
    Int,
    Double,
    String
}

// Enum representing the different types of operations
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Opcode {
    Mul, // multiplication (*)
    Div, // division (/)
    Add, // addition (+)
    Sub, // subtraction (-)
}

