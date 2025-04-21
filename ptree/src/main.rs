mod frontend;
mod backend;

use frontend::token::Token;
use lalrpop_util::lalrpop_mod;
use logos::Logos;

//Custom imports
use crate::backend::backend::compile;

lalrpop_mod!(pub calculator4);

//Print the tokens received after lexical analysis
fn print_tokens(tokens: &[(usize, Token, usize)]) {
    print!("Tokens: ");
    for (_, token, _) in tokens {
        print!("{:?} ", token);
    }
    println!();
}

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
    
    //Print tokens
    print_tokens(&tokens);

    let parser = calculator4::ExprParser::new();
    match parser.parse(tokens.into_iter()) {
        Ok(ast) => {
            println!("Parse Tree: {:#?}", ast.to_raw());
            compile(&ast);
        }
        Err(e) => {
            eprintln!("Parse error: {:?}", e);
        }
    }
}