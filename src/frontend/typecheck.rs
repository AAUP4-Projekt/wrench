// Import HashMap to keep track of variable types and their types
use std::collections::{HashMap, HashSet};
// Import the AST types
use super::ast::{
    ColumnAssignmentEnum, Declaration, Expr, Operator, Parameter, Statement, TypeConstruct,
    TypedExpr,
};

/// Structure to hold information about a variable
/// - `var_type`: The declared type of the variable
/// - `is_constant`: Whether the variable is immutable
#[derive(PartialEq, Debug, Clone)]
pub struct VariableInfo {
    pub var_type: TypeConstruct,
    pub is_constant: bool,
}

// Main function to perform type checking on a statement
// - `statement`: The statement to type check
// - `scope_stack`: A mutable reference to the stack of variable scopes (used for scoping rules)
pub fn type_check(
    statement: &Statement,
    scope_stack: &mut Vec<HashMap<String, VariableInfo>>,
) -> Result<(), String> {
    // Match on the type of statement to handle different cases
    match statement {
        // Case: Skip statement (no operation)
        Statement::Skip => {
            // Skip statement, do nothing
        }

        // Case: Compound statement - Check both parts of a compound statement
        Statement::Compound(stmt1, stmt2) => {
            type_check(stmt1, scope_stack)?;
            type_check(stmt2, scope_stack)?;
        }

        // Case: Variable declaration - Handle different types of declarations
        Statement::Declaration(declaration) => {
            match declaration {
                // Case: Variable declaration with a type, name, and expression
                Declaration::Variable(var_type, name, expr) => {
                    // Check and cast the type of the expression
                    check_and_cast_type(
                        &(VariableInfo {
                            var_type: var_type.clone(),
                            is_constant: false,
                        }),
                        expr,
                        scope_stack,
                    )?;
                    // Add variable to the current scope
                    scope_stack.last_mut().unwrap().insert(
                        name.clone(),
                        VariableInfo {
                            var_type: var_type.clone(),
                            is_constant: false,
                        },
                    );
                }
                // Case: Constant declaration with a type, name, and expression
                Declaration::Constant(const_type, name, expr) => {
                    // Check and cast the type of the expression
                    let typed_expr = infer_type(expr, scope_stack)?;
                    if *const_type != typed_expr.expr_type {
                        return Err(format!(
                            "Type mismatch: expected {:?}, found {:?} for constant '{}'",
                            const_type, typed_expr.expr_type, name
                        ));
                    }
                    // Add the constant to the current scope
                    scope_stack.last_mut().unwrap().insert(
                        name.clone(),
                        VariableInfo {
                            var_type: const_type.clone(),
                            is_constant: true,
                        },
                    );
                }
                // Case: Function declaration with a return type, name, parameters, and body
                Declaration::Function(return_type, name, params, body) => {
                    let param_types: Vec<TypeConstruct> = params
                        .iter()
                        .map(|Parameter::Parameter(param_type, _)| param_type.clone())
                        .collect();

                    scope_stack[0].insert(
                        name.clone(),
                        VariableInfo {
                            var_type: TypeConstruct::Function(
                                Box::new(return_type.clone()),
                                param_types,
                            ),
                            is_constant: true,
                        },
                    );

                    // Create a scope for the function parameters
                    let mut param_scope = HashMap::new();
                    for Parameter::Parameter(param_type, param_name) in params {
                        param_scope.insert(
                            param_name.clone(),
                            VariableInfo {
                                var_type: param_type.clone(),
                                is_constant: false,
                            },
                        );
                    }

                    // Preserve previously declared functions
                    let mut function_scope = HashMap::new();
                    for (k, v) in scope_stack[0].iter() {
                        if matches!(v.var_type, TypeConstruct::Function(_, _)) {
                            function_scope.insert(k.clone(), v.clone());
                        }
                    }

                    let mut function_scope_stack = Vec::new();
                    function_scope_stack.push(function_scope);
                    function_scope_stack.push(param_scope);

                    type_check(body, &mut function_scope_stack)?;
                }
            }
        }

        // Case: For loop
        Statement::For(param, iterable_expr, body) => {
            let typed_iterable = infer_type(iterable_expr, scope_stack)?;

            // Match on the type of the iterable expression
            match &typed_iterable.expr_type {
                TypeConstruct::Array(element_type) => {
                    push_scope(scope_stack);

                    // Match on the parameter type
                    match param {
                        Parameter::Parameter(param_type, param_name) => {
                            if *param_type != **element_type {
                                return Err(format!(
                                    "Type mismatch in for-loop: expected {:?}, found {:?} for iterator '{}'",
                                    param_type, element_type, param_name
                                ));
                            }
                            scope_stack.last_mut().unwrap().insert(
                                param_name.clone(),
                                VariableInfo {
                                    var_type: *element_type.clone(),
                                    is_constant: false,
                                },
                            );
                        }
                    }

                    type_check(body, scope_stack)?;

                    pop_scope(scope_stack);
                }
                TypeConstruct::Row(_) => {
                    push_scope(scope_stack);

                    // Match on the parameter type
                    match param {
                        Parameter::Parameter(param_type, param_name) => {
                            if *param_type != typed_iterable.expr_type {
                                return Err(format!(
                                    "Type mismatch in for-loop: expected {:?}, found {:?} for iterator '{}'",
                                    param_type, typed_iterable.expr_type, param_name
                                ));
                            }
                            scope_stack.last_mut().unwrap().insert(
                                param_name.clone(),
                                VariableInfo {
                                    var_type: typed_iterable.expr_type.clone(),
                                    is_constant: false,
                                },
                            );
                        }
                    }

                    type_check(body, scope_stack)?;

                    pop_scope(scope_stack);
                }
                TypeConstruct::Table(table_params) => {
                    push_scope(scope_stack);
                    match param {
                        Parameter::Parameter(param_type, param_name) => {
                            if let TypeConstruct::Row(row_params) = param_type {
                                if row_params != table_params {
                                    return Err(format!(
                                        "Type mismatch in for-loop: expected Row({:?}), found Table({:?}) for iterator '{}'",
                                        row_params, table_params, param_name
                                    ));
                                }
                            } else {
                                return Err(format!(
                                    "Type mismatch in for-loop: expected Row(...), found Table({:?}) for iterator '{}'",
                                    table_params, param_name
                                ));
                            }
                            scope_stack.last_mut().unwrap().insert(
                                param_name.clone(),
                                VariableInfo {
                                    var_type: param_type.clone(),
                                    is_constant: false,
                                },
                            );
                        }
                    }
                    type_check(body, scope_stack)?;
                    pop_scope(scope_stack);
                }
                _ => {
                    return Err(format!(
                        "For-loop iterable must be an array, found {:?}",
                        typed_iterable.expr_type
                    ));
                }
            }
        }

        // Case: Variable assignment
        Statement::VariableAssignment(name, expr) => {
            if let Some(var_type) = lookup_variable(name, scope_stack) {
                if var_type.is_constant {
                    return Err(format!("Cannot assign to constant variable '{}'", name));
                }

                check_and_cast_type(&var_type, expr, scope_stack)?;
                // Update the variable type in the current scope
                scope_stack
                    .last_mut()
                    .unwrap()
                    .insert(name.clone(), var_type.clone());
            } else {
                return Err(format!("Undefined variable '{}'", name));
            }
        }

        // Case: Constant assignment
        Statement::Expr(expr) => {
            infer_type(expr, scope_stack)?;
        }

        // Case: If statement
        Statement::If(condition, body, else_body) => {
            let typed_condition = infer_type(condition, scope_stack)?;
            if typed_condition.expr_type != TypeConstruct::Bool {
                return Err("If condition must be a boolean".to_string());
            }

            // Push a new scope for the if body
            push_scope(scope_stack);
            type_check(body, scope_stack)?;
            pop_scope(scope_stack);

            // Push a new scope for the else body
            push_scope(scope_stack);
            type_check(else_body, scope_stack)?;
            pop_scope(scope_stack);
        }

        // Case: While statement
        Statement::While(condition, body) => {
            let typed_condition = infer_type(condition, scope_stack)?;
            if typed_condition.expr_type != TypeConstruct::Bool {
                return Err("While condition must be a boolean".to_string());
            }

            // Push a new scope for the while body
            push_scope(scope_stack);
            type_check(body, scope_stack)?;
            pop_scope(scope_stack);
        }

        // Case: return statement
        Statement::Return(expr) => {
            infer_type(expr, scope_stack)?;
        }
    }

    Ok(())
}

// Function to infer the type of an expression
fn infer_type(
    expr: &Expr,
    scope_stack: &mut Vec<HashMap<String, VariableInfo>>,
) -> Result<TypedExpr, String> {
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
        Expr::StringLiteral(value) => Ok(TypedExpr {
            expr: Expr::StringLiteral(value.clone()),
            expr_type: TypeConstruct::String,
        }),

        // Case: Null literal (e.g., `null`)
        Expr::Null => Ok(TypedExpr {
            expr: Expr::Null,
            expr_type: TypeConstruct::Null,
        }),

        // Case: Identifier (e.g., `x`)
        Expr::Identifier(name) => {
            if let Some(var_info) = lookup_variable(name, scope_stack) {
                Ok(TypedExpr {
                    expr: Expr::Identifier(name.clone()),
                    expr_type: var_info.var_type.clone(),
                })
            } else {
                Err(format!("Undefined variable '{}'", name))
            }
        }

        // Case: Binary operation (e.g., `x + y`)
        Expr::Operation(left, op, right) => {
            let left_typed = infer_type(left, scope_stack)?;
            let right_typed = infer_type(right, scope_stack)?;

            // Check if the operator is valid for the types
            let widened_left = check_and_cast_type(
                &VariableInfo {
                    var_type: right_typed.expr_type.clone(),
                    is_constant: false,
                },
                &left_typed.expr,
                scope_stack,
            )?;
            let widened_right = check_and_cast_type(
                &VariableInfo {
                    var_type: left_typed.expr_type.clone(),
                    is_constant: false,
                },
                &right_typed.expr,
                scope_stack,
            )?;

            if matches!(left_typed.expr_type, TypeConstruct::Row(_))
                || matches!(right_typed.expr_type, TypeConstruct::Row(_))
                || matches!(left_typed.expr_type, TypeConstruct::Table(_))
                || matches!(right_typed.expr_type, TypeConstruct::Table(_))
            {
                return Err("Operation on Row or Table types is not allowed".to_string());
            }

            // Determine the result type based on the operator and operand types
            let result_type = match (&left_typed.expr_type, &right_typed.expr_type) {
                (TypeConstruct::Int, TypeConstruct::Double)
                | (TypeConstruct::Double, TypeConstruct::Int)
                | (TypeConstruct::Double, TypeConstruct::Double) => TypeConstruct::Double,
                (TypeConstruct::Int, TypeConstruct::Int) => TypeConstruct::Int,
                _ => {
                    return Err(format!(
                        "Operation on incompatible types. Left-hand side is {:?} and right-hand side is {:?}",
                        left_typed.expr_type, right_typed.expr_type
                    ));
                }
            };

            // Only allow arithmetic operations on Int or Double
            match op {
                Operator::Equals | Operator::LessThan | Operator::LessThanOrEqual => {
                    Ok(TypedExpr {
                        expr: Expr::Operation(
                            Box::new(widened_left),
                            (*op).clone(),
                            Box::new(widened_right),
                        ),
                        expr_type: TypeConstruct::Bool,
                    })
                }
                Operator::Addition
                | Operator::Subtraction
                | Operator::Multiplication
                | Operator::Division
                | Operator::Modulo
                | Operator::Exponent => {
                    if result_type == TypeConstruct::Int || result_type == TypeConstruct::Double {
                        // Check for division by zero
                        if let Operator::Division = op {
                            match &right_typed.expr {
                                Expr::Number(0) | Expr::Double(0.0) => {
                                    return Err("Division by zero is not allowed".to_string());
                                }
                                _ => {}
                            }
                        }
                        Ok(TypedExpr {
                            expr: Expr::Operation(
                                Box::new(widened_left),
                                (*op).clone(),
                                Box::new(widened_right),
                            ),
                            expr_type: result_type,
                        })
                    } else {
                        Err(format!("Invalid operation for type {:?}", result_type))
                    }
                }
                Operator::Or => {
                    if left_typed.expr_type == TypeConstruct::Bool
                        && right_typed.expr_type == TypeConstruct::Bool
                    {
                        Ok(TypedExpr {
                            expr: Expr::Operation(
                                Box::new(widened_left),
                                (*op).clone(),
                                Box::new(widened_right),
                            ),
                            expr_type: TypeConstruct::Bool,
                        })
                    } else {
                        Err("Logical operators require boolean operands".to_string())
                    }
                }
            }
        }
        // Case: Logical NOT (e.g., `!true`)
        Expr::Not(inner) => {
            let inner_typed = infer_type(inner, scope_stack)?;
            if inner_typed.expr_type == TypeConstruct::Bool {
                Ok(TypedExpr {
                    expr: Expr::Not(Box::new(inner_typed.expr)),
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

            let first_typed = infer_type(&elements[0], scope_stack)?;
            // Ensure all elements in the array have the same type
            for e in elements.iter().skip(1) {
                let t = infer_type(e, scope_stack)?;
                if t.expr_type != first_typed.expr_type {
                    return Err("Array elements must have the same type".to_string());
                }
            }
            // Build the array expression with typed elements
            Ok(TypedExpr {
                expr: Expr::Array(
                    elements
                        .iter()
                        .map(|e| infer_type(e, scope_stack).map(|typed| Box::new(typed.expr)))
                        .collect::<Result<Vec<_>, _>>()?,
                ),
                expr_type: TypeConstruct::Array(Box::new(first_typed.expr_type)),
            })
        }

        // Case: Indexing (e.g., `arr[0]`)
        Expr::Indexing(array_expr, index_expr) => {
            let array_typed = infer_type(array_expr, scope_stack)?;
            let index_typed = infer_type(index_expr, scope_stack)?;

            if index_typed.expr_type != TypeConstruct::Int {
                return Err("Index must be an integer".to_string());
            }

            // Make sure we're indexing into an array
            match array_typed.expr_type {
                TypeConstruct::Array(inner) => Ok(TypedExpr {
                    expr: Expr::Indexing(Box::new(array_typed.expr), Box::new(index_typed.expr)),
                    expr_type: *inner,
                }),

                TypeConstruct::Row(_) => Ok(TypedExpr {
                    expr: Expr::Indexing(Box::new(array_typed.expr), Box::new(index_typed.expr)),
                    expr_type: array_typed.expr_type.clone(),
                }),

                TypeConstruct::Table(_) => Ok(TypedExpr {
                    expr: Expr::Indexing(Box::new(array_typed.expr), Box::new(index_typed.expr)),
                    expr_type: array_typed.expr_type.clone(),
                }),
                _ => Err("Cannot index into non-array type".to_string()),
            }
        }

        // Case for function call (e.g., `f(x, y)`)
        Expr::FunctionCall(name, args) => {
            if let Some(func_type) = lookup_variable(name, scope_stack) {
                if let TypeConstruct::Function(return_type, param_types) = &func_type.var_type {
                    if args.len() != param_types.len() {
                        return Err(format!(
                            "Function '{}' expected {} arguments, found {}",
                            name,
                            param_types.len(),
                            args.len()
                        ));
                    }

                    for (i, (arg, param_type)) in args.iter().zip(param_types.iter()).enumerate() {
                        let arg_typed = infer_type(arg, scope_stack)?;
                        // Tillad alle tabeller som argument til import
                        if (name == "import" || name == "async_import") && i == 1 {
                            if let (TypeConstruct::Table(_), TypeConstruct::Table(_)) =
                                (param_type, &arg_typed.expr_type)
                            {
                                continue;
                            }
                        }
                        if *param_type != TypeConstruct::Any && arg_typed.expr_type != *param_type {
                            return Err(format!(
                                "Type mismatch in function call: expected {:?}, found {:?}",
                                param_type, arg_typed.expr_type
                            ));
                        }
                    }

                    // Returnér korrekt tabeltype for import
                    if name == "import" || name == "async_import" {
                        if let Some(arg) = args.get(1) {
                            if let Expr::Table(params) = &**arg {
                                return Ok(TypedExpr {
                                    expr: Expr::FunctionCall(name.clone(), args.clone()),
                                    expr_type: TypeConstruct::Table(params.clone()),
                                });
                            }
                        }
                    }

                    Ok(TypedExpr {
                        expr: Expr::FunctionCall(name.clone(), args.clone()),
                        expr_type: *return_type.clone(),
                    })
                } else {
                    Err(format!("'{}' is not a function", name))
                }
            } else {
                Err(format!("Undefined function '{}'", name))
            }
        }

        // Case: pipe operation (e.g., `x pipe f`)
        Expr::Pipe(left, pipe_name, args) => {
            let left_typed = infer_type(left, scope_stack)?;
            println!("Pipe input type: {:?}", left_typed.expr_type);

            if let Some(func_type) = lookup_variable(pipe_name, scope_stack) {
                if let TypeConstruct::Function(return_type, param_types) = &func_type.var_type {
                    let effective_args: Vec<Expr> = if args.is_empty() && param_types.len() == 1 {
                        vec![*Box::new(left_typed.expr.clone())]
                    } else {
                        args.iter().map(|b| (*b.clone())).collect()
                    };

                    if effective_args.len() != param_types.len() {
                        return Err(format!(
                            "Pipe function '{}' expected {} arguments, found {}",
                            pipe_name,
                            param_types.len(),
                            args.len()
                        ));
                    }

                    let input_type = &left_typed.expr_type;
                    let output_type = &**return_type;

                    let allowed = match (input_type, output_type) {
                        (TypeConstruct::Row(cols_in), TypeConstruct::Row(cols_out)) => {
                            cols_in == cols_out
                        }
                        (TypeConstruct::Row(cols_in), TypeConstruct::Bool) => {
                            if let TypeConstruct::Row(cols_param) = &param_types[0] {
                                cols_in == cols_param
                            } else {
                                false
                            }
                        }
                        (TypeConstruct::Table(cols_in), TypeConstruct::Table(cols_out)) => {
                            cols_in == cols_out
                        }
                        _ => false,
                    };

                    if !allowed {
                        return Err(format!(
                            "Pipe function '{}' must be one of: Row->Row (map), Row->Bool (filter), Table->Table (reduce) with matching columns. Got: {:?} -> {:?}",
                            pipe_name, input_type, output_type
                        ));
                    }
                    Ok(TypedExpr {
                        expr: Expr::Pipe(
                            Box::new(left_typed.expr),
                            pipe_name.clone(),
                            args.clone(),
                        ),
                        expr_type: *return_type.clone(),
                    })
                } else {
                    Err(format!("'{}' is not a valid pipe function", pipe_name))
                }
            } else {
                Err(format!("Undefined pipe function '{}'", pipe_name))
            }
        }

        // Case: table
        Expr::Table(params) => {
            let mut param_types = Vec::new();
            let mut seen_names = HashSet::new();

            for param in params {
                match param {
                    Parameter::Parameter(param_type, param_name) => {
                        // Check for duplicate parameter names
                        if !seen_names.insert(param_name.clone()) {
                            return Err(format!(
                                "Duplicate parameter name '{}' in table declaration",
                                param_name
                            ));
                        }
                        param_types
                            .push(Parameter::Parameter(param_type.clone(), param_name.clone()));
                    }
                }
            }

            Ok(TypedExpr {
                expr: Expr::Table(params.clone()),
                expr_type: TypeConstruct::Table(param_types),
            })
        }

        // Case: row
        Expr::Row(column_assignments) => {
            let mut param_types = Vec::new();
            for column in column_assignments {
                // Match on the type of column assignment
                match column {
                    ColumnAssignmentEnum::ColumnAssignment(param_type, param_name, expr) => {
                        let typed_expr = infer_type(expr, scope_stack)?;
                        if *param_type != typed_expr.expr_type {
                            return Err(format!(
                                "Type mismatch: expected {:?}, found {:?} for column '{}'",
                                param_type, typed_expr.expr_type, param_name
                            ));
                        }
                        param_types
                            .push(Parameter::Parameter(param_type.clone(), param_name.clone()));
                    }
                }
            }
            Ok(TypedExpr {
                expr: Expr::Row(column_assignments.clone()),
                expr_type: TypeConstruct::Row(param_types),
            })
        }

        // Case: column indexing
        Expr::ColumnIndexing(table_expr, column_name) => {
            let table_typed = infer_type(table_expr, scope_stack)?;

            match &table_typed.expr_type {
                TypeConstruct::Table(params) | TypeConstruct::Row(params) => {
                    for Parameter::Parameter(col_type, col_name) in params {
                        if col_name == column_name {
                            return Ok(TypedExpr {
                                expr: Expr::ColumnIndexing(
                                    Box::new(table_typed.expr),
                                    column_name.clone(),
                                ),
                                expr_type: col_type.clone(),
                            });
                        }
                    }
                    Err(format!(
                        "Column '{}' not found in {:?}",
                        column_name, table_typed.expr_type
                    ))
                }
                _ => Err("Cannot index into non-table/row type".to_string()),
            }
        }
    }
}

// Helper function to look up a variable in the scope stack
pub fn lookup_variable(
    name: &str,
    scope_stack: &[HashMap<String, VariableInfo>],
) -> Option<VariableInfo> {
    for scope in scope_stack.iter().rev() {
        if let Some(var_info) = scope.get(name) {
            return Some(var_info.clone());
        }
    }
    None
}

// Helper function to push a new scope onto the stack
// Push means to add a new element to the end of the vector
fn push_scope(scope_stack: &mut Vec<HashMap<String, VariableInfo>>) {
    scope_stack.push(HashMap::new());
}

// Helper function to pop the current scope off the stack
// Pop means to remove the last element from the vector
fn pop_scope(scope_stack: &mut Vec<HashMap<String, VariableInfo>>) {
    scope_stack.pop();
}

// Helper function to check and cast types
fn check_and_cast_type(
    expected_type: &VariableInfo,
    expr: &Expr,
    scope_stack: &mut Vec<HashMap<String, VariableInfo>>,
) -> Result<Expr, String> {
    let typed_expr = infer_type(expr, scope_stack)?;

    match (&expected_type.var_type, &typed_expr.expr_type) {
        // Implicit cast from Int to Double allowed
        (TypeConstruct::Double, TypeConstruct::Int) => Ok(typed_expr.expr.clone()),
        // Implicit cast from Double to Int not allowed
        (TypeConstruct::Int, TypeConstruct::Double) => Err(format!(
            "Cannot implicitly cast Double to Int. Expected {:?}, found {:?}",
            expected_type, typed_expr.expr_type
        )),
        // If the expected type matches the inferred type
        _ if expected_type.var_type == typed_expr.expr_type => Ok(typed_expr.expr),
        // If the types do not match, return an error
        _ => Err(format!(
            "Type mismatch: expected {:?}, found {:?}",
            expected_type, typed_expr.expr_type
        )),
    }
}

/*
//Unit-integration tests:
#[cfg(test)]
mod tests {
    use std::result;

    use super::*;
    use crate::frontend::main::create_syntax_tree;

    //type casting unit tests
    #[test]
    fn test_legal_int_plus_double_implicit() {
        let aritmoperation = "var int a = 5; var double b = 4.5; a + b;";
        let tree = create_syntax_tree(aritmoperation);
        let result = type_check(&tree);
        assert!(result.is_ok(), "int + double is allowed");
    }
    #[test]
    fn test_legal_double_plus_int_implicit() {
        let aritmoperation = "var double a = 3.5; var int b = 4; a + b;";
        let tree = create_syntax_tree(aritmoperation);
        let result = type_check(&tree);
        assert!(result.is_ok(), "double + int is allowed");
    }
    #[test]
    fn test_legal_int_minus_double_implicit() {
        let aritmoperation = "var int a = 5; var double b = 4.5; a - b;";
        let tree = create_syntax_tree(aritmoperation);
        let result = type_check(&tree);
        assert!(result.is_ok(), "int - double is allowed");
    }
    #[test]
    fn test_legal_double_minus_int_implicit() {
        let aritmoperation = "var double a = 3.5; var int b = 4; a - b;";
        let tree = create_syntax_tree(aritmoperation);
        let result = type_check(&tree);
        assert!(result.is_ok(), "double - int is allowed");
    }
    #[test]
    fn test_legal_int_times_double_implicit() {
        let aritmoperation = "var int a = 5; var double b = 4.5; a * b;";
        let tree = create_syntax_tree(aritmoperation);
        let result = type_check(&tree);
        assert!(result.is_ok(), "int * double is allowed");
    }
    #[test]
    fn test_legal_double_times_int_implicit() {
        let aritmoperation = "var double a = 3.5; var int b = 4; a * b;";
        let tree = create_syntax_tree(aritmoperation);
        let result = type_check(&tree);
        assert!(result.is_ok(), "double * int is allowed");
    }
    #[test]
    fn test_legal_int_slash_double_implicit() {
        let aritmoperation = "var int a = 5; var double b = 4.5; a / b;";
        let tree = create_syntax_tree(aritmoperation);
        let result = type_check(&tree);
        assert!(result.is_ok(), "int / double is allowed");
    }

    #[test]
    fn test_legal_double_slash_int_implicit() {
        let aritmoperation = "var double a = 3.5; var int b = 4; a / b;";
        let tree = create_syntax_tree(aritmoperation);
        let result = type_check(&tree);
        assert!(result.is_ok(), "double / int is allowed");
    }

    //Legal Explicit type casting

    #[test]
    fn test_legal_explicit_double_to_int() {
        let source = "var double num1 = 5.4; var int num2 = (int) num1;";
        let tree = create_syntax_tree(source);
        let result = type_check(&tree);
        assert!(
            result.is_ok(),
            "Explicit coercion from double to int successful"
        );
    }

    #[test]
    fn test_legal_explicit_int_to_double() {
        let source = "var int num1 = 5; var double num2 = (double) num1;";
        let tree = create_syntax_tree(source);
        let result = type_check(&tree);
        assert!(
            result.is_ok(),
            "Explicit coercion from int to double successful"
        );
    }

    //Illegal implicit narrow typecasting

    #[test]
    fn test_illegal_implicit_narrowing() {
        let code = "var double a = 7.35; var int b = a;";
        let tree = create_syntax_tree(code);
        let result = type_check(&tree);
        assert!(result.is_err(), "You cannot implicitly narrow a double!"); //assert will get a bool, not an option
    }

    // String + String is not allowed!

    #[test]
    fn test_illegal_string_plus_string() {
        let source = "var string mystring1 = \"Hello\"; var string mystring2 = \"World\"; var string result = mystring1 + mystring2;";
        let tree = create_syntax_tree(source);
        let result = type_check(&tree);
        assert!(result.is_err(), "String concatenation is not allowed!");
    }

    #[test]
    fn test_illegal_int_plus_string() {
        let source = r#"
        var int myinteger = 10;
        var string mystring = "Hello?";
        var int result = myinteger + mystring;
    "#;
        let tree = create_syntax_tree(source);
        let result = type_check(&tree);

        assert!(result.is_err(), "You cannot perform int + string");
    }

    #[test]
    fn test_illegal_assign_string_to_int() {
        let source = r#"var int x = "Hello World";"#;
        let tree = create_syntax_tree(source);
        let result = type_check(&tree);

        assert!(
            result.is_err(),
            "Type of variable does not match expression type! Assignment cannot be performed."
        );
    }

    #[test]
    fn test_illegal_bool_operation() {
        let source = r#"
        var int x = 5000;
        var string y = "Aalborg University";
        var bool a = x <= y;
        var bool b = x == y;
        var bool c = x or y;
        var bool d = x and y;
        "#;
        let tree = create_syntax_tree(source);
        let result = type_check(&tree);

        assert!(
            result.is_err(),
            "Boolean operation on incompatible types not allowed!"
        );
    }

    #[test]
    fn test_illegal_array_index() {
        let source = r#" var bool index = true; var string array[] myfruits = ["apple", "banana", "strawberry"]; var string lastfruit = myfruits[index];"#;
        let tree = create_syntax_tree(source);
        let result = type_check(&tree);

        assert!(
            result.is_err(),
            "Index type is not compatible with operation!"
        );
    }

    #[test]
    fn test_illegal_if_branch() {
        let source = r#" var int x = 1 ; var string mystring = "candy"; if (mystring) {x + 1} "#;
        let tree = create_syntax_tree(source);
        let result = type_check(&tree);
        assert!(
            result.is_err(),
            "In if statement the conditional needs to be boolean!"
        );
    }

    #[test]
    fn test_illegal_if_branch_2() {
        let source =
            r#" var bool condition = true ; var int myint = 100 ; if (condition) {x = "Hi"} "#;
        let tree = create_syntax_tree(source);
        let result = type_check(&tree);
        assert!(
            result.is_err(),
            "Illegal operation in the if branch! Type mismatch. "
        );
    }

    #[test]
    fn test_illegal_constant_mod() {
        let source = r#"
        const int count = 0;
        fn int f() {
            count = count + 1;
        }
        "#;
        let tree = create_syntax_tree(source);
        let result = type_check(&tree);
        assert!(result.is_err(), "Cannot change value of const!")
    }
}
*/
