use frontend::frontend::create_ast;

mod frontend;
mod backend;


//use frontend::token::Token;
//use lalrpop_util::lalrpop_mod;
//use logos::Logos;

//Custom imports
//use crate::backend::backend::compile;

//lalrpop_mod!(pub calculator4);
/*
//Print the tokens received after lexical analysis
fn print_tokens(tokens: &[(usize, Token, usize)]) {
    print!("Tokens: ");
    for (_, token, _) in tokens {
        print!("{:?} ", token);
    }
    println!();
}
    */

//#[cfg(not(test))]
fn main() {
    let input = "3 + 5 * (2 ** 3)";
    create_ast(input);
}