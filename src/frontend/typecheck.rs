use std::collections::HashMap;
use super::ast::{Expr, Statement, TypeConstruct};

pub fn type_check(statements: &[Statement]) -> Result<(), String> {
    let mut symbol_table: HashMap<String, TypeConstruct> = HashMap::new();

    for statement in statements {
        match statement {
            Statement::VariableDeclaration(var_type, name, expr) => {
                let expr_type = infer_type(expr, &symbol_table)?;
                if *var_type != expr_type {
                    return Err(format!(
                        "Type mismatch: expected {:?}, found {:?} for variable '{}'",
                        var_type, expr_type, name
                    ));
                }
                symbol_table.insert(name.clone(), var_type.clone());
            }
            Statement::VariableAssignment(name, expr) => {
                if let Some(var_type) = symbol_table.get(name) {
                    let expr_type = infer_type(expr, &symbol_table)?;
                    if *var_type != expr_type {
                        return Err(format!(
                            "Type mismatch: expected {:?}, found {:?} for variable '{}'",
                            var_type, expr_type, name
                        ));
                    }
                } else {
                    return Err(format!("Undefined variable '{}'", name));
                }
            }
            Statement::Expr(expr) => {
                infer_type(expr, &symbol_table)?;
            }
        }
    }

    Ok(())
}

fn infer_type(expr: &Expr, symbol_table: &HashMap<String, TypeConstruct>) -> Result<TypeConstruct, String> {
    match expr {
        Expr::Number(_) => Ok(TypeConstruct::Int),
        Expr::Bool(_) => Ok(TypeConstruct::Bool),
        Expr::Identifier(name) => {
            symbol_table.get(name).cloned().ok_or_else(|| format!("Undefined variable '{}'", name))
        }
        Expr::Operation(left, operator, right) => {
            let left_type = infer_type(left, symbol_table)?;
            let right_type = infer_type(right, symbol_table)?;
            if left_type != right_type {
                return Err(format!(
                    "Type mismatch in operation: left is {:?}, right is {:?}",
                    left_type, right_type
                ));
            }
            match operator {
                _ if left_type == TypeConstruct::Int => Ok(TypeConstruct::Int),
                _ => Err(format!("Unsupported operator for type {:?}", left_type)),
            }
        }
    }
}