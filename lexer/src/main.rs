//Define enum

use logos::Logos;

#[derive(Logos, Debug, PartialEq)]

enum Token {
    // Define tokens here:  Based on our abstract syntax add types, and different symbols

    //Let us define keywords first
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

    #[token("true")]
    True,

    #[token("false")]
    False,

    #[token("<")] //FOR GENERICS to be handled at parsing, not lexical analysis
    LeftAngle,

    #[token(">")] //FOR GENERICS to be handled at parsing, not lexical analysis
    RightAngle,

    #[token("if")]
    If,

    #[token("else")]
    Else,

    //Definitions from statements
    #[token("var")]
    Var,

    #[token("skip")] //equals to "pass" in python
    Skip,

    //Symbols from arithmetic expressions
    #[token("**")]
    DoubleStar,

    #[token("*")]
    Star,

    #[token("==")]
    EqualsOperator,

    #[token("=")]
    AssignmentOperator,

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
    IntegerNumbers,

    //Double numbers
    #[regex(r"[0-9]+\.[0-9]+")]
    DoubleNumbers,

    //An array of string literals: or words like "cat", "dog", "computer science"
    #[regex(r#""([^"\\]|\\.)*""#)]
    //Whitespace - Ignore whitespace. \t is tab, \n is newline \f is form feed
    #[regex(r"[ \t\n\f]+", logos::skip)]
    Whitespace,
}

fn main() {
    let mut lex = Token::lexer("My mom said that 24 == 6 * 4");

    while let Some(token) = lex.next() {
        //Going through all tokens
        println!("{:?} : {:?}", token, lex.slice());
    }
}
