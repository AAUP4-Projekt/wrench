use crate::frontend::ast::Expr;

pub fn wrench_print(args: Vec<Expr>) -> Expr{

    for arg in args {
        match arg {
            Expr::Number(num) => println!("{}", num),
            Expr::Double(num) => println!("{}", num),
            Expr::StringLiteral(s) => println!("{}", s),
            Expr::Bool(b) => println!("{}", b),
            _ => println!("Unsupported expression type for print"),
        }
    }
    Expr::Null
}
