#![allow(clippy::vec_box)]

use std::fmt::Debug;

#[derive(PartialEq, Debug)]
pub struct TypedExpr {
    pub expr: Expr,               // Represents the expression itself               
    pub expr_type: TypeConstruct, // Represents the type of the expression
}

#[derive(PartialEq, Debug)]
pub enum Statement {
    Expr(Box<Expr>),                       // Represents an expression statement
    VariableAssignment(String, Box<Expr>), // Represents a variable assignment with its name and value
    Declaration(Declaration),              // Represents a declaration
    Return(Option<Box<Expr>>), // Represents a return statement with an optional expression
    If(Box<Expr>, Vec<Statement>, Option<Vec<Statement>>), // Represents an if statement with its condition, body, and optional else body
    For(Parameter, Box<Expr>, Vec<Statement>), // Represents a for loop with its initialization, condition, and body
    While(Box<Expr>, Vec<Statement>), // Represents a while loop with its condition and body
}

#[derive(PartialEq, Debug)]
pub enum Declaration {
    Variable(TypeConstruct, String, Box<Expr>), // Represents a variable declaration with its type, name, and assigned value
    Constant(TypeConstruct, String, Box<Expr>), // Represents a variable declaration with its type, name, and assigned value
    Function(TypeConstruct, String, Vec<Parameter>, Vec<Statement>), // Represents a function declaration with its return type, name, parameters, and body
}

#[derive(PartialEq, Debug)]
pub enum Expr {
    Number(i32),                               // Represents a number
    Double(f64),                               // Represents a double value
    Null,                                      // Represents a null value
    StringLiteral(String),                     // Represents a string literal
    Identifier(String),                        // Represents an identifier (variable name)
    Bool(bool),                                // Represents a boolean value
    Operation(Box<Expr>, Operator, Box<Expr>), // Represents an operation with left and right operands and an operator
    Not(Box<Expr>), // Represents a unary operation with an operator and an operand
    Table(Vec<Parameter>),
    Row(Vec<ColumnAssignmentEnum>),
    Indexing(Box<Expr>, Box<Expr>), // Represents indexing, e.g. into an array
    Array(Vec<Box<Expr>>),          // Represents an array with its elements
    Pipe(Box<Expr>, String, Vec<Box<Expr>>), // Represents a pipe operation, e.g. for chaining operations
    FunctionCall(String, Vec<Box<Expr>>), // Represents a function call with its name and arguments
    ColumnIndexing(Box<Expr>, Box<Expr>), // Represents indexing into a column of a table or row
}

// Enum representing types
#[derive(PartialEq, Debug, Clone)]
pub enum TypeConstruct {
    Bool,
    Int,
    Double,
    String,
    Null,
    Generic(String),              // Represents a generic type with a name
    Optional(Box<TypeConstruct>), // Represents an optional type
    Array(Box<TypeConstruct>),    // Represents an array type
    Function(Box<TypeConstruct>, Vec<TypeConstruct>), // Represents a function type with return type and parameter types
    Table(Vec<Parameter>),                            // Represents a table type with its columns
    Row(Vec<Parameter>),                              // Represents a row type with its columns
}

// Enum representing the different types of operations
#[derive(PartialEq, Debug)]
pub enum Operator {
    Multiplication,     // multiplication (*)
    Exponent,           // exponent (**)
    Addition,           // addition (+)
    Subtraction,        // subtraction (-)
    Division,           // division (/)
    Modulo,             // modulo (%)
    Equals,             // equality (==)
    LessThan,           // less than (<)
    GreaterThan,        // greater than (>)
    LessThanOrEqual,    // less than or equal (<=)
    GreaterThanOrEqual, // greater than or equal (>=)
    And,                // logical AND
    Or,                 // logical OR
}

/*
=======================================
Building blocks, used in other enums
=======================================
*/

#[derive(PartialEq, Debug)]
pub enum Parameter {
    Parameter(TypeConstruct, String), // Represents a parameter with its type and name
}

#[derive(PartialEq, Debug)]
pub enum ColumnAssignmentEnum {
    ColumnAssignment(TypeConstruct, String, Box<Expr>), // Represents a column assignment with its type, name, and value
}
