use std::fmt::Debug;

#[derive(PartialEq, Debug)]
pub struct TypedExpr {
    pub expr: Expr,               // Represents the expression itself               
    pub expr_type: TypeConstruct, // Represents the type of the expression
}

#[derive(PartialEq, Debug)]
pub enum Statement {
    Expr(Box<TypedExpr>), // Represents an expression statement
    VariableDeclaration(TypeConstruct, String, Box<TypedExpr>), // Represents a variable declaration with its type, name, and assigned value
    VariableAssignment(String, Box<TypedExpr>), // Represents a variable assignment with its name and value
}

#[derive(PartialEq, Debug)]
pub enum Expr {
    Number(i32),                               // Represents a number
    Identifier(String),                        // Represents an identifier (variable name)
    Bool(bool),                                // Represents a boolean value
    Double(f64),                               // Represents a double-precision floating-point number
    String(String),                            // Represents a string
    Not(Box<TypedExpr>),                       // Represents a logical NOT operation
    Array(Vec<TypedExpr>),                     // Represents an array
    Index(Box<TypedExpr>, Box<TypedExpr>),     // Represents array indexing
    Operation(Box<TypedExpr>, Operator, Box<TypedExpr>), // Represents an operation with left and right operands and an operator
}

// Enum representing types
#[derive(PartialEq, Debug, Clone)]
pub enum TypeConstruct {
    Bool,
    Int,
    Double,
    String,
    Array(Box<TypeConstruct>),              
}

// Enum representing the different types of operations
#[derive(PartialEq, Debug)]
pub enum Operator {
    Mul, // multiplication (*)
    Exp, // exponent (**)
    Add, // addition (+)
    Sub, // subtraction (-)
}
