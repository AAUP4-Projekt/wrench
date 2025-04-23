use std::fmt::{Debug, Error, Formatter};

pub enum Statement {
    Expr(Box<Expr>), // Represents an expression statement
    VariableDeclaration(TypeConstruct, String, Box<Expr>), // Represents a variable declaration with its type, name, and assigned value
    VariableAssignment(String, Box<Expr>), // Represents a variable assignment with its name and value
}

// The 'Expr' enum represents the abstract syntax tree (AST)
pub enum Expr {
    Number(i32), // Represents a number
    Identifier(String), // Represents an identifier (variable name)
    Bool(bool), // Represents a boolean value
    Op(Box<Expr>, Opcode, Box<Expr>), // Represents an operation with left and right operands and an operator
}

// Enum representing types
#[derive(Copy, Clone)]
pub enum TypeConstruct {
    Bool,
    Int,
    Double,
    String
}

// Enum representing the different types of operations
#[derive(Copy, Clone)]
pub enum Opcode {
    Mul, // multiplication (*)
    Div, // division (/)
    Add, // addition (+)
    Sub, // subtraction (-)
}
/*
// Custom 'Debug' implementation for pretty-printing the 'Expr' nodes
/// Output might look like: `(1 + (2 * 3))`
impl Debug for Expr {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> Result<(), Error> {
        use self::Expr::*;
        match *self {
            // Print the number directly
            Number(n) => write!(fmt, "{:?}", n),

            // Print the operation with its operands and operator
            Op(ref l, op, ref r) => write!(fmt, "({:?} {:?} {:?})", l, op, r),

        }
    }
}
    */

// Custom 'Debug' implementation for pretty-printing the 'Opcode' enum
/// This will print symbols like `+`, `-`, `*`, `/` instead of enum names.
impl Debug for Opcode {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> Result<(), Error> {
        use self::Opcode::*;
        match *self {
            Mul => write!(fmt, "*"),
            Div => write!(fmt, "/"),
            Add => write!(fmt, "+"),
            Sub => write!(fmt, "-"),
        }
    }
}

impl Debug for TypeConstruct {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> Result<(), Error> {
        use self::TypeConstruct::*;
        match *self {
            Bool => write!(fmt, "Bool"),
            Int => write!(fmt, "Int"),
            Double => write!(fmt, "Double"),
            String => write!(fmt, "String"),
        }
    }
}


impl Statement {
    pub fn to_raw(&self) -> String {
        let x = match self {
            Statement::Expr(expr) => format!("Expr(Box::new({}))", expr.to_raw()),
            Statement::VariableDeclaration(t, x, e) => format!("VariableDelcaration({:?}, {}, Box::new({}))", t, x, e.to_raw()),
            Statement::VariableAssignment(x, e) => format!("VariableAssignment({}, Box::new({}))", x, e.to_raw()),
        };
        return format!("Statement({})", x);
    }
}

// Implementation block for methods on 'Expr'
impl Expr {
    // This converts the 'Expr' into a raw string representation that can be used for debugging or logging
    /// Output example:
    /// `Op(Box::new(Number(3)), +, Box::new(Op(Box::new(Number(5)), *, Box::new(Number(2))))`
    pub fn to_raw(&self) -> String {
        match self {
            Expr::Number(n) => format!("Number({})", n),

            Expr::Bool(b) => format!("Bool({})", b),

            // Recursively convert the left and right operands to raw strings
            Expr::Op(left, op, right) => {
                format!(
                    "Op(Box::new({}), {:?}, Box::new({}))",
                    left.to_raw(),
                    op,
                    right.to_raw()
                )
            }

            Expr::Identifier(name) => format!("Identifier({})", name),

            // Display "Error" for errors
            //Expr::Error => "Error".to_string(),
        }
    }

    /*
    // Recursively evaluates the AST and returns the result, or and error message if the expression is invalid
    // It can look something like this: `Ok(5)` or `Err("Division by zero")`
    pub fn evaluate(&self) -> Result<i32, String> {
        match self {
            Expr::Number(n) => Ok(*n),

            // Evaluate both sides and apply the operator
            Expr::Op(left, op, right) => {
                let l_val = left.evaluate()?;
                let r_val = right.evaluate()?;
                match op {
                    Opcode::Add => Ok(l_val + r_val),
                    Opcode::Sub => Ok(l_val - r_val),
                    Opcode::Mul => Ok(l_val * r_val),
                    Opcode::Div => {
                        if r_val == 0 {
                            Err("Division by zero".to_string())
                        } else {
                            Ok(l_val / r_val)
                        }
                    }
                }
            }

            // Return an error if trying to evalute a broken expression
            //Expr::Error => Err("Invalid expression".to_string()),
        }
    }
    */
}

