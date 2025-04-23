use lalrpop_util::lalrpop_mod;
use logos::Logos;

use super::lexer::Token;

lalrpop_mod!(pub grammar);

pub fn create_ast(input: &str) {

    //Lex
    let lexer = Token::lexer(input);
    
    //Collect tokens
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

    let parser = grammar::ProgramParser::new();

    match parser.parse(tokens.into_iter()) {
        Ok(program) => {
            for statement in program{
                println!("Parse tree: {:#?}", statement.to_raw());
            }
        }
        Err(e) => {
            eprintln!("Parse error: {:?}", e);
        }
    }
    
}