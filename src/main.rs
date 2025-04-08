//Define enum

use logos::Logos;

#[derive(Logos, Debug, PartialEq)]

enum Token {
    // Define tokens here:  Based on our abstract syntax add types, and different symbols

    //Let us define our keywords first
    #[token("bool")]
    Boolean,

    #[token("int")]
    IntegerKeyword,

    #[token("double")]
    Double,

    #[token("string")]
    String,

    #[token("null")]
    Null,

    #[token("<x>")]
    Generic,

    #[token("if")]
    If,

    #[token("else")]
    Else,

    //Symbols from arithmetic expressions, and statements
    #[token("=")]
    AssignmentOperator,

    #[token("==")]
    EqualsOperator,

    #[token("+")]
    Plus,

    #[token("-")]
    Minus,

    #[token("/")]
    Slash,

    #[token(";")]
    Semicolon,

    #[token("!")]
    ExclamationMark,

    #[token("?")]
    QuestionMark,

    // Identifiers for variables, or function names: First symbol can be lower/uppcaseletter or underscore. Second symbol same, and can be repeated a number of times
    #[regex("[a-zA-Z_][a-zA-Z_]*")]
    Identifier,

    //Integer numbers
    #[regex("[0-9]+")]
    //Integer must be at least of length 1, can be anything constructed from 0 to 9
    IntegerNumber,

    //Whitespace
    #[regex(r"[ \t\n\f]+", logos::skip)]
    //Ignore whitespace. \t is tab, \n is newline \f is form feed
    Whitespace,
}

fn main() {
    let mut lex = Token::lexer("I love puppies!");

    while let Some(token) = lex.next() {
        //Going through all tokens
        println!("{:?} : {:?}", token, lex.slice());
    }
}
