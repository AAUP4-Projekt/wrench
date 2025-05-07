// Import HashMap to keep track of variable types and their types
use std::collections::HashMap;
// Import the AST types
use super::ast::{
    ColumnAssignmentEnum, Declaration, Expr, Operator, Parameter, Statement, TypeConstruct,
    TypedExpr,
};

// This function checks the types of a list of statements
pub fn type_check(statements: &[Statement]) -> Result<Vec<Statement>, String> {
    // This stack of scopes keeps track of variable names and their types
    let mut scope_stack: Vec<HashMap<String, TypeConstruct>> = vec![HashMap::new()];
    // This will hold the new, type-annotated version of each statement
    let mut typed_statements = Vec::new();

    // Add built-in functions to the global scope
    scope_stack[0].insert(
        "print".to_string(),
        TypeConstruct::Function(
            Box::new(TypeConstruct::Null), // Return type of `print` is `Null`
            vec![TypeConstruct::Int],      // `print` takes one `Int` argument
        ),
    );

    // Go through each statement in the list
    for statement in statements {
        println!("Evaluating statement: {:?}", statement);
        match statement {
            // Case 1: Variable declaration
            Statement::Declaration(declaration) => {
                match declaration {
                    Declaration::Variable(var_type, name, expr) => {
                        let typed_expr = infer_type(expr, &scope_stack)?;
                        if *var_type != typed_expr.expr_type {
                            return Err(format!(
                                "Type mismatch: expected {:?}, found {:?} for variable '{}'",
                                var_type, typed_expr.expr_type, name
                            ));
                        }
                        // Add the variable to the current scope
                        scope_stack
                            .last_mut()
                            .unwrap()
                            .insert(name.clone(), var_type.clone());
                        // Add the type-annotated declaration to the result list
                        typed_statements.push(Statement::Declaration(Declaration::Variable(
                            var_type.clone(),
                            name.clone(),
                            Box::new(typed_expr.expr),
                        )));
                    }
                    Declaration::Constant(const_type, name, expr) => {
                        let typed_expr = infer_type(expr, &scope_stack)?;
                        if *const_type != typed_expr.expr_type {
                            return Err(format!(
                                "Type mismatch: expected {:?}, found {:?} for constant '{}'",
                                const_type, typed_expr.expr_type, name
                            ));
                        }
                        // Add the constant to the current scope
                        scope_stack
                            .last_mut()
                            .unwrap()
                            .insert(name.clone(), const_type.clone());
                        // Add the type-annotated constant to the result list
                        typed_statements.push(Statement::Declaration(Declaration::Constant(
                            const_type.clone(),
                            name.clone(),
                            Box::new(typed_expr.expr),
                        )));
                    }
                    Declaration::Function(return_type, name, params, body) => {
                        let param_types: Vec<TypeConstruct> = params
                            .iter()
                            .map(|Parameter::Parameter(param_type, _)| param_type.clone())
                            .collect();
                        scope_stack.last_mut().unwrap().insert(
                            name.clone(),
                            TypeConstruct::Function(Box::new(return_type.clone()), param_types),
                        );

                        // Push a new scope for the function body
                        push_scope(&mut scope_stack);
                        for Parameter::Parameter(param_type, param_name) in params {
                            scope_stack
                                .last_mut()
                                .unwrap()
                                .insert(param_name.clone(), param_type.clone());
                        }

                        let mut typed_body = Vec::new();
                        for stmt in body {
                            let typed_stmt = type_check(&[stmt.clone()])?;
                            typed_body.extend(typed_stmt);
                        }

                        // Pop the function scope
                        pop_scope(&mut scope_stack);

                        // Add the type-annotated function to the result list
                        typed_statements.push(Statement::Declaration(Declaration::Function(
                            return_type.clone(),
                            name.clone(),
                            params.clone(),
                            typed_body,
                        )));
                    }
                }
            }

            // Case for
            Statement::For(param, iterable_expr, body) => {
                let typed_iterable = infer_type(iterable_expr, &scope_stack)?;

                // Match på typen af `typed_iterable.expr_type`
                match &typed_iterable.expr_type {
                    TypeConstruct::Array(element_type) => {
                        push_scope(&mut scope_stack);

                        // Match på parameteren
                        match param {
                            Parameter::Parameter(param_type, param_name) => {
                                if *param_type != **element_type {
                                    return Err(format!(
                                        "Type mismatch in for-loop: expected {:?}, found {:?} for iterator '{}'",
                                        param_type, element_type, param_name
                                    ));
                                }
                                scope_stack
                                    .last_mut()
                                    .unwrap()
                                    .insert(param_name.clone(), *element_type.clone());
                            }
                        }

                        let mut typed_body = Vec::new();
                        for stmt in body {
                            let typed_stmt = type_check(&[stmt.clone()])?;
                            typed_body.extend(typed_stmt);
                        }

                        pop_scope(&mut scope_stack);

                        typed_statements.push(Statement::For(
                            param.clone(),
                            Box::new(typed_iterable.expr),
                            typed_body,
                        ));
                    }
                    _ => {
                        return Err(format!(
                            "For-loop iterable must be an array, found {:?}",
                            typed_iterable.expr_type
                        ));
                    }
                }
            }

            // Case Variable assignment
            Statement::VariableAssignment(name, expr) => {
                if let Some(var_type) = lookup_variable(name, &scope_stack) {
                    let typed_expr = infer_type(expr, &scope_stack)?;
                    if var_type != typed_expr.expr_type {
                        return Err(format!(
                            "Type mismatch: expected {:?}, found {:?} for variable '{}'",
                            var_type, typed_expr.expr_type, name
                        ));
                    }
                    typed_statements.push(Statement::VariableAssignment(
                        name.clone(),
                        Box::new(typed_expr.expr),
                    ));
                } else {
                    return Err(format!("Undefined variable '{}'", name));
                }
            }

            Statement::Expr(expr) => {
                let typed_expr = infer_type(expr, &scope_stack)?;
                typed_statements.push(Statement::Expr(Box::new(typed_expr.expr)));
            }

            // Case: If statement
            Statement::If(condition, body, else_body) => {
                let typed_condition = infer_type(condition, &scope_stack)?;
                if typed_condition.expr_type != TypeConstruct::Bool {
                    return Err("If condition must be a boolean".to_string());
                }

                // Push a new scope for the if body
                push_scope(&mut scope_stack);
                let mut typed_body = Vec::new();
                for stmt in body {
                    let typed_stmt = type_check(&[stmt.clone()])?;
                    typed_body.extend(typed_stmt);
                }
                pop_scope(&mut scope_stack);

                // Handle the else body if it exists
                let mut typed_else_body = None;
                if let Some(else_body_statements) = else_body {
                    push_scope(&mut scope_stack);
                    let mut typed_else = Vec::new();
                    for stmt in else_body_statements {
                        let typed_stmt = type_check(&[stmt.clone()])?;
                        typed_else.extend(typed_stmt);
                    }
                    pop_scope(&mut scope_stack);
                    typed_else_body = Some(typed_else);
                }

                // Add the type-annotated if statement to the result list
                typed_statements.push(Statement::If(
                    Box::new(typed_condition.expr),
                    typed_body,
                    typed_else_body,
                ));
            }

            // Case: While statement
            Statement::While(condition, body) => {
                let typed_condition = infer_type(condition, &scope_stack)?;
                if typed_condition.expr_type != TypeConstruct::Bool {
                    return Err("While condition must be a boolean".to_string());
                }

                // Push a new scope for the while body
                push_scope(&mut scope_stack);
                let mut typed_body = Vec::new();
                for stmt in body {
                    let typed_stmt = type_check(&[stmt.clone()])?;
                    typed_body.extend(typed_stmt);
                }
                pop_scope(&mut scope_stack);

                // Add the type-annotated while statement to the result list
                typed_statements.push(Statement::While(Box::new(typed_condition.expr), typed_body));
            }

            Statement::Return(expr) => {
                let typed_expr = match expr {
                    Some(e) => Some(infer_type(e, &scope_stack)?),
                    None => None,
                };
                typed_statements.push(Statement::Return(typed_expr.map(|e| Box::new(e.expr))));
            }
        }
    }

    Ok(typed_statements)
}

// Update `infer_type` to use the scope stack
fn infer_type(
    expr: &Expr,
    scope_stack: &[HashMap<String, TypeConstruct>],
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

        Expr::Identifier(name) => {
            let expr_type = lookup_variable(name, scope_stack)
                .ok_or_else(|| format!("Undefined variable '{}'", name))?;
            Ok(TypedExpr {
                expr: Expr::Identifier(name.clone()),
                expr_type,
            })
        }

        // Case: Binary operation (e.g., `x + y`)
        Expr::Operation(left, op, right) => {
            let left_typed = infer_type(left, scope_stack)?;
            let right_typed = infer_type(right, scope_stack)?;

            // Make sure both sides have the same type todo: NEEDS TO BE CHANGED
            if left_typed.expr_type != right_typed.expr_type {
                return Err(format!(
                    "Type mismatch in operation: left is {:?}, right is {:?}",
                    left_typed.expr_type, right_typed.expr_type
                ));
            }

            // Only allow arithmetic operations on Int or Double
            match op {
                Operator::Addition
                | Operator::Subtraction
                | Operator::Multiplication
                | Operator::Division
                | Operator::Exponent => {
                    if left_typed.expr_type == TypeConstruct::Int
                        || left_typed.expr_type == TypeConstruct::Double
                    {
                        Ok(TypedExpr {
                            expr: Expr::Operation(
                                Box::new(left_typed.expr),
                                (*op).clone(),
                                Box::new(right_typed.expr),
                            ),
                            expr_type: left_typed.expr_type,
                        })
                    } else {
                        Err(format!(
                            "Invalid operation for type {:?}",
                            left_typed.expr_type
                        ))
                    }
                }
                _ => Err("Unsupported operator".to_string()),
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
                _ => Err("Cannot index into non-array type".to_string()),
            }
        }

        // Case for function call (e.g., `f(x, y)`)
        Expr::FunctionCall(name, args) => {
            if let Some(func_type) = lookup_variable(name, scope_stack) {
                if let TypeConstruct::Function(return_type, param_types) = func_type {
                    if args.len() != param_types.len() {
                        return Err(format!(
                            "Function '{}' expected {} arguments, found {}",
                            name,
                            param_types.len(),
                            args.len()
                        ));
                    }

                    // Check argument types
                    for (arg, param_type) in args.iter().zip(param_types.iter()) {
                        let arg_typed = infer_type(arg, scope_stack)?;
                        if arg_typed.expr_type != *param_type {
                            return Err(format!(
                                "Type mismatch in function call: expected {:?}, found {:?}",
                                param_type, arg_typed.expr_type
                            ));
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

        // Case for pipe operation (e.g., `x pipe f`)
        Expr::Pipe(left, pipe_name, args) => {
            // Type-check input to the pipe
            let left_typed = infer_type(left, scope_stack)?;
            println!("Pipe input type: {:?}", left_typed.expr_type);

            // Check if the input to the pipe is either Row or Table
            match left_typed.expr_type {
                TypeConstruct::Row(_) | TypeConstruct::Table(_) => {
                    // Input is valid, proceed
                }
                _ => {
                    return Err(format!(
                        "Pipe operation requires input of type Row or Table, found {:?}",
                        left_typed.expr_type
                    ));
                }
            }

            // Look up the function being piped to
            if let Some(func_type) = lookup_variable(pipe_name, scope_stack) {
                if let TypeConstruct::Function(return_type, param_types) = func_type {
                    // Check if the number of arguments matches the function's parameters
                    if args.len() != param_types.len() {
                        return Err(format!(
                            "Function '{}' expected {} arguments, found {}",
                            pipe_name,
                            param_types.len(),
                            args.len()
                        ));
                    }

                    // Check argument types
                    for (arg, param_type) in args.iter().zip(param_types.iter()) {
                        let arg_typed = infer_type(arg, scope_stack)?;
                        if arg_typed.expr_type != *param_type {
                            return Err(format!(
                                "Type mismatch in function call: expected {:?}, found {:?}",
                                param_type, arg_typed.expr_type
                            ));
                        }
                    }

                    // Check if the output of the pipe is either Row or Table
                    match *return_type {
                        TypeConstruct::Row(_) | TypeConstruct::Table(_) => {
                            // Output is valid
                            Ok(TypedExpr {
                                expr: Expr::Pipe(
                                    Box::new(left_typed.expr),
                                    pipe_name.clone(),
                                    args.clone(),
                                ),
                                expr_type: *return_type.clone(),
                            })
                        }
                        _ => Err(format!(
                            "Pipe operation must return type Row or Table, found {:?}",
                            return_type
                        )),
                    }
                } else {
                    Err(format!("'{}' is not a function", pipe_name))
                }
            } else {
                Err(format!("Undefined function '{}'", pipe_name))
            }
        }

        // Case for table (e.g `table [int x, double y]`)
        Expr::Table(params) => {
            let mut param_types = Vec::new();
            for param in params {
                match param {
                    Parameter::Parameter(param_type, param_name) => {
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

        // Case for row
        Expr::Row(column_assignments) => {
            let mut param_types = Vec::new();
            for column in column_assignments {
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

        // Case for column indexing
        Expr::ColumnIndexing(table_expr, column_expr) => {
            let table_typed = infer_type(table_expr, scope_stack)?;
            let column_typed = infer_type(column_expr, scope_stack)?;

            // Check if the table is of type Table or Row
            match table_typed.expr_type {
                TypeConstruct::Table(_) | TypeConstruct::Row(_) => {
                    // Check if the column is of type String
                    if column_typed.expr_type != TypeConstruct::String {
                        return Err("Column name must be a string".to_string());
                    }
                    Ok(TypedExpr {
                        expr: Expr::ColumnIndexing(
                            Box::new(table_typed.expr),
                            Box::new(column_typed.expr),
                        ),
                        expr_type: table_typed.expr_type.clone(), // Return the same type as the table
                    })
                }
                _ => Err("Cannot index into non-table/row type".to_string()),
            }
        }
    }
}

// Helper function to look up a variable in the scope stack
pub fn lookup_variable(
    name: &str,
    scope_stack: &[HashMap<String, TypeConstruct>],
) -> Option<TypeConstruct> {
    for scope in scope_stack.iter().rev() {
        if let Some(var_type) = scope.get(name) {
            return Some(var_type.clone());
        }
    }
    None
}

// Helper function to push a new scope onto the stack
fn push_scope(scope_stack: &mut Vec<HashMap<String, TypeConstruct>>) {
    scope_stack.push(HashMap::new());
}

// Helper function to pop the current scope off the stack
fn pop_scope(scope_stack: &mut Vec<HashMap<String, TypeConstruct>>) {
    scope_stack.pop();
}

//Unit-integration tests:
#[cfg(test)]
mod tests {
    use super::*;
    use crate::frontend::main::create_syntax_tree;

    fn type_error(source: &str, expected_error: &str) {
        //Parse the source code into an AST. IMPROTANT: I am assuming that parsing is correct. I only check for type errors
        let tree = create_syntax_tree(source);
        let type_annotated_tree = type_check(&tree);

        //Assert error
        assert!(type_annotated_tree.is_err(), "Typecheck passed");
        let error = type_annotated_tree.err().unwrap();

        assert!(
            error.contains(expected_error),
            "The program expected this error message : '{}', but got : '{}'",
            expected_error,
            error
        );
    }

    #[test]
    fn test_incompatible_type() {
        type_error("var int myfirstinteger = 'Hello World';", "Type mismatch");
    }
}
