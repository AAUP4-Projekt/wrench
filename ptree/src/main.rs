mod token;
mod ast;

use token::Token;
use lalrpop_util::lalrpop_mod;
use logos::Logos;
use crate::ast::{Expr, Opcode};

lalrpop_mod!(pub calculator4);

#[cfg(not(test))]
fn main() {
    let input = "3 + 3 * 2 + 5 * (2 - 1)";

    let lexer = Token::lexer(input);

    let tokens: Vec<_> = lexer
        .spanned()
        .filter_map(|(token, span)| match token {
            Ok(t) => Some((span.start, t, span.end)),
            Err(_) => {
                eprintln!("Invalid token at {:?}", span);
                None
            },
        })
        .collect();

    let parser = calculator4::ExprParser::new();
    match parser.parse(tokens.into_iter()) {
        Ok(ast) => {
            println!("Parse Tree: {:#?}", ast.to_raw());

            let mut counter = 0;
            let mut buffer = Vec::new(); 
            let result_register = generate_llvm_ir(&ast, &mut counter, &mut buffer);

            for line in buffer {
                println!("{}", line);
            }
            println!("Result is in register: {}", result_register);
        }
        Err(e) => {
            eprintln!("Parse error: {:?}", e);
        }
    }
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
