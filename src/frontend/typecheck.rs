// Import HashMap to keep track of variable types and their types
use std::collections::HashMap;
// Import the AST types
use super::ast::{Expr, Operator, Statement, TypeConstruct, TypedExpr};

// This function checks the types of a list of statements
pub fn type_check(statements: &[Statement]) -> Result<Vec<Statement>, String> {
    // This symbol table keeps track of each variable's name and its type
    let mut symbol_table: HashMap<String, TypeConstruct> = HashMap::new();
    // This will hold the new, type-annotated version of each statement
    let mut typed_statements = Vec::new();

    // Go through each statement in the list
    for statement in statements {
        match statement {

            // Case 1: Variable declaration
            Statement::VariableDeclaration(var_type, name, expr) => {
                // Try to figure out the type of the expression
                let typed_expr = infer_type(&expr.expr, &symbol_table)?;
                // If the declared type doesn't match the inferred type, return an error
                if *var_type != typed_expr.expr_type {
                    return Err(format!(
                        "Type mismatch: expected {:?}, found {:?} for variable '{}'",
                        var_type, typed_expr.expr_type, name
                    ));
                }
                // Save the variable's type in the symbol table
                symbol_table.insert(name.clone(), var_type.clone());
                 // Add the type-annotated declaration to the result list
                typed_statements.push(Statement::VariableDeclaration(
                    var_type.clone(),
                    name.clone(),
                    Box::new(typed_expr),
                ));
            }

            // Case 2: Variable assignment
            Statement::VariableAssignment(name, expr) => {
                // Check if the variable was declared
                if let Some(var_type) = symbol_table.get(name) {
                    // Infer the type of the expression on the right-hand side
                    let typed_expr = infer_type(&expr.expr, &symbol_table)?;
                    // Check if the types match
                    if *var_type != typed_expr.expr_type {
                        return Err(format!(
                            "Type mismatch: expected {:?}, found {:?} for variable '{}'",
                            var_type, typed_expr.expr_type, name
                        ));
                    }
                    // Add the type-annotated assignment to the result list
                    typed_statements.push(Statement::VariableAssignment(
                        name.clone(),
                        Box::new(typed_expr),
                    ));
                } else {
                    // If the variable doesn't exist, return an error
                    return Err(format!("Undefined variable '{}'", name));
                }
            }
            // Case 3: Expression statement (e.g., just a function call or value on its own)
            Statement::Expr(expr) => {
                let typed_expr = infer_type(&expr.expr, &symbol_table)?;
                typed_statements.push(Statement::Expr(Box::new(typed_expr)));
            }
        }
    }

    // If everything was OK, return the new list of type-annotated statements
    Ok(typed_statements)
}


// This function figures out the type of an expression
fn infer_type(expr: &Expr, symbol_table: &HashMap<String, TypeConstruct>) -> Result<TypedExpr, String> {
    match expr {
        // Case: Integer literal (e.g., `5`)
        Expr::Number(value) => Ok(TypedExpr {
            expr: Expr::Number(*value),
            expr_type: TypeConstruct::Int,
        }),
        // Case: Boolean literal (e.g., `true`)
        Expr::Bool(value) => Ok(TypedExpr {
            expr: Expr::Bool(*value),
            expr_type: TypeConstruct::Bool,
        }),
         // Case: Floating-point number (e.g., `3.14`)
        Expr::Double(value) => Ok(TypedExpr {
            expr: Expr::Double(*value),
            expr_type: TypeConstruct::Double,
        }),
        // Case: String literal (e.g., `"hello"`)
        Expr::String(value) => Ok(TypedExpr {
            expr: Expr::String(value.clone()),
            expr_type: TypeConstruct::String,
        }),
        // Case: Variable reference (e.g., `x`)
        Expr::Identifier(name) => {
            let expr_type = symbol_table
                .get(name)
                .cloned()
                .ok_or_else(|| format!("Undefined variable '{}'", name))?;
            Ok(TypedExpr {
                expr: Expr::Identifier(name.clone()),
                expr_type,
            })
        }
        // Case: Binary operation (e.g., `x + y`)
        Expr::Operation(left, op, right) => {
            let left_typed = infer_type(left, symbol_table)?;
            let right_typed = infer_type(right, symbol_table)?;

            // Make sure both sides have the same type todo: NEEDS TO BE CHANGED
            if left_typed.expr_type != right_typed.expr_type {
                return Err(format!(
                    "Type mismatch in operation: left is {:?}, right is {:?}",
                    left_typed.expr_type, right_typed.expr_type
                ));
            }

            // Only allow arithmetic operations on Int or Double
            match op {
                Operator::Add | Operator::Sub | Operator::Mul | Operator::Div | Operator::Exp => {
                    if left_typed.expr_type == TypeConstruct::Int || left_typed.expr_type == TypeConstruct::Double {
                        Ok(TypedExpr {
                            expr: Expr::Operation(Box::new(left_typed), *op, Box::new(right_typed)),
                            expr_type: left_typed.expr_type,
                        })
                    } else {
                        Err(format!("Invalid operation for type {:?}", left_typed.expr_type))
                    }
                }
                _ => Err("Unsupported operator".to_string()),
            }
        }

        // Case: Logical NOT (e.g., `!true`)
        Expr::Not(inner) => {
            let inner_typed = infer_type(inner, symbol_table)?;
            if inner_typed.expr_type == TypeConstruct::Bool {
                Ok(TypedExpr {
                    expr: Expr::Not(Box::new(inner_typed)),
                    expr_type: TypeConstruct::Bool,
                })
            } else {
                Err("Logical NOT requires a boolean".to_string())
            }
        }

        // Case: Array (e.g., `[1, 2, 3]`)
        Expr::Array(elements) => {
            if elements.is_empty() {
                return Err("Cannot infer type of empty array".to_string());
            }

            let first_typed = infer_type(&elements[0], symbol_table)?;
            // Ensure all elements in the array have the same type
            for e in elements.iter().skip(1) {
                let t = infer_type(e, symbol_table)?;
                if t.expr_type != first_typed.expr_type {
                    return Err("Array elements must have the same type".to_string());
                }
            }
            // Build the array expression with typed elements
            Ok(TypedExpr {
                expr: Expr::Array(
                    elements
                        .iter()
                        .map(|e| infer_type(e, symbol_table))
                        .collect::<Result<Vec<_>, _>>()?,
                ),
                expr_type: TypeConstruct::Array(Box::new(first_typed.expr_type)),
            })
        }

        // Case: Indexing (e.g., `arr[0]`)
        Expr::Index(array_expr, index_expr) => {
            let array_typed = infer_type(array_expr, symbol_table)?;
            let index_typed = infer_type(index_expr, symbol_table)?;

            if index_typed.expr_type != TypeConstruct::Int {
                return Err("Index must be an integer".to_string());
            }

            // Make sure we're indexing into an array
            match array_typed.expr_type {
                TypeConstruct::Array(inner) => Ok(TypedExpr {
                    expr: Expr::Index(Box::new(array_typed), Box::new(index_typed)),
                    expr_type: *inner,
                }),
                _ => Err("Cannot index into non-array type".to_string()),
            }
        }
        // Catch-all for anything unsupported
        _ => Err("Unsupported expression".to_string()),
    }
}
