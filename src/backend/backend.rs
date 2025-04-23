//use crate::frontend::{Expr, Opcode};

/*
use crate::frontend::ast::Expr;
use crate::frontend::ast::Opcode;

pub fn compile(ast: &Expr){
    let mut counter = 0;
    let mut buffer = Vec::new(); 
    let result_register = generate_llvm_ir(&ast, &mut counter, &mut buffer);

    for line in buffer {
        println!("{}", line);
    }
    println!("Result is in register: {}", result_register);
}


fn generate_llvm_ir(ast: &Expr, counter: &mut u32, buffer: &mut Vec<String>) -> String {
    match *ast {
        Expr::Number(n) => format!("{}", n),
        Expr::Op(ref left, op, ref right) => {
            let left_ir = generate_llvm_ir(left, counter, buffer);
            let right_ir = generate_llvm_ir(right, counter, buffer);

            let op_ir = match op {
                Opcode::Add => "add i32",
                Opcode::Sub => "sub i32",
                Opcode::Mul => "mul i32",
                Opcode::Div => "div i32",
            };

            let reg_name = format!("%{}", *counter);
            *counter += 1;

            let op_result = format!("{} = {} {}, {}", reg_name, op_ir, left_ir, right_ir);
            buffer.push(op_result);

            reg_name
        },
        _ => String::from("error"),
    }
}
*/