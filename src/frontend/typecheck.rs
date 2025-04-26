// Import HashMap to keep track of variable types and their types
use std::collections::HashMap;
// Import the AST types
use super::ast::{Expr, Statement, TypeConstruct};

// This function checks the types of a list of statements
pub fn type_check(statements: &[Statement]) -> Result<(), String> {
    // Create a symbol table to keep track of variable types
    // The symbol table is a HashMap that maps variable names to their types
    let mut symbol_table: HashMap<String, TypeConstruct> = HashMap::new();

    // Go through each statement in the list of statements
    for statement in statements {
        match statement {
            // Handle variable declarations
            Statement::VariableDeclaration(var_type, name, expr) => {
                // Infer the type of the expression on the right side of the assignment
                let expr_type = infer_type(expr, &symbol_table)?;
                // Check if the declared type matches the actual type
                if *var_type != expr_type {
                    // If they don't match, return an error message
                    // The error message includes the expected type, found type, and variable name
                    return Err(format!(
                        "Type mismatch: expected {:?}, found {:?} for variable '{}'",
                        var_type, expr_type, name
                    ));
                }
                // Add the variable to the symbol table with its type
                symbol_table.insert(name.clone(), var_type.clone());
            }
            // Handle variable assignments
            Statement::VariableAssignment(name, expr) => {
                // Make sure the variable was declared before
                if let Some(var_type) = symbol_table.get(name) {
                    // Infer the type of the new value
                    let expr_type = infer_type(expr, &symbol_table)?;
                    // Check if the new value matches the variable's type
                    if *var_type != expr_type {
                        return Err(format!(
                            "Type mismatch: expected {:?}, found {:?} for variable '{}'",
                            var_type, expr_type, name
                        ));
                    }
                } else {
                    // If the variable was not declared, return an error message
                    // The error message includes the variable name
                    return Err(format!("Undefined variable '{}'", name));
                }
            }
            // Handle standalone expressions (e.g, function calls)
            Statement::Expr(expr) => {
                // Just infer the type to make sure the expression is valid
                infer_type(expr, &symbol_table)?;
            }
        }
    }

    // If all statements are fine, return success
    Ok(())
}


// This function figures out the type of an expression
fn infer_type(expr: &Expr, symbol_table: &HashMap<String, TypeConstruct>) -> Result<TypeConstruct, String> {
    match expr {
        // Numbers are always integers
        Expr::Number(_) => Ok(TypeConstruct::Int),
        // Booleans are always booleans
        Expr::Bool(_) => Ok(TypeConstruct::Bool),
        // For identifiers (variables), look up their type in the symbol table
        Expr::Identifier(name) => {
            symbol_table.get(name).cloned().ok_or_else(|| format!("Undefined variable '{}'", name))
        }
        // For operations like addition or multiplication, check the types of the left and right operands
        // and make sure they match
        Expr::Operation(left, operator, right) => {
            // Infer the type of both the left and right expressions
            let left_type = infer_type(left, symbol_table)?;
            let right_type = infer_type(right, symbol_table)?;
            // Make sure both sides of the operation are of the same type
            if left_type != right_type {
                return Err(format!(
                    "Type mismatch in operation: left is {:?}, right is {:?}",
                    left_type, right_type
                ));
            }
            // Allow operations only if the types are integers
            match operator {
                _ if left_type == TypeConstruct::Int => Ok(TypeConstruct::Int),
                _ => Err(format!("Unsupported operator for type {:?}", left_type)),
            }
        }
    }
}