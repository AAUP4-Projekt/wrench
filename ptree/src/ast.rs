use std::fmt::{Debug, Error, Formatter};

pub enum Expr {
    Number(i32),
    Op(Box<Expr>, Opcode, Box<Expr>),
    Error,
}

#[derive(Copy, Clone)]
pub enum Opcode {
    Mul, 
    Div,
    Add, 
    Sub,
}

/// Debug implementation for `Expr` to pretty-print the AST
impl Debug for Expr {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> Result<(), Error> {
        use self::Expr::*;
        match *self {
            Number(n) => write!(fmt, "{:?}", n),
            Op(ref l, op, ref r) => write!(fmt, "({:?} {:?} {:?})", l, op, r),
            Error => write!(fmt, "error"),
        }
    }
}

/// Debug implementation for `Opcode` to display operators
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

/// Optional: Add evaluation logic for the AST
impl Expr {
    pub fn to_raw(&self) -> String {
        match self {
            Expr::Number(n) => format!("Number({})", n),
            Expr::Op(left, op, right) => {
                format!(
                    "Op(Box::new({}), {:?}, Box::new({}))",
                    left.to_raw(),
                    op,
                    right.to_raw()
                )
            }
            Expr::Error => "Error".to_string(),
        }
    }
    pub fn evaluate(&self) -> Result<i32, String> {
        match self {
            Expr::Number(n) => Ok(*n),
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
            Expr::Error => Err("Invalid expression".to_string()),
        }
    }
}

