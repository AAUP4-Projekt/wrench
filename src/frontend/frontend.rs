
//use super::token::print_tokens;

use super::lexer::Token;

pub fn create_ast(input: &str) {
    let lexer = Token::lexer(input);
    //let tokens = Token::collect_tokens(lexer);
    //print_tokens(&tokens);
}