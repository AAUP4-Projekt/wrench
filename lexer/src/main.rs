//Define enum

use logos::Logos;

#[derive(Logos, Debug, PartialEq)]

enum Token {
    //First things first
    //Whitespace - Ignore whitespace. \t is tab, \n is newline \f is form feed
    #[regex(r"[ \t\n\f]+", logos::skip)]
    Whitespace,

    #[error] //error handler
    Error,

    #[regex(r"//[^\n]*", logos::skip)] //ignore oneline comments like this one

    // Identifiers variables, or function names
    #[regex("[a-zA-Z_][a-zA-Z_]*")]
    Identifier,

    //Operators
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

    //Constants
    #[regex("[0-9]+")]
    IntegerNumbers,

    #[regex(r"[0-9]+\.[0-9]+")]
    DoubleNumbers,

    //Keywords
    #[token("bool")]
    Boolean,

    #[token("int")]
    IntegerKeyword,

    #[token("double")]
    Double,

    #[token("string")]
    String,

    #[token("table")]
    Table,

    #[token("row")]
    Row,

    #[token("fn")]
    Function,

    #[token("var")]
    Var,

    #[token("const")]
    Constant,

    #[token("null")]
    Null,

    #[token("true")]
    True,

    #[token("false")]
    False,

    #[token("if")]
    If,

    #[token("else")]
    Else,

    #[token("while")]
    While,

    #[token("skip")] //equals to "pass" in python
    Skip,

    //Literals
    #[regex(r#""([^"\\]|\\.)*""#)]
    Stringliteral,

    //Punctuators
    #[token(";")]
    Semicolon,

    #[token("(")]
    Openparan,

    #[token(")")]
    Closeparan,

    #[token("}")]
    Opencurlybracket,

    #[token("}")]
    Closecurlybracket,

    #[token("[]")]
    Opensquarebracket,

    #[token("]")]
    Closesquarebracket,

    #[token("<")] //FOR GENERICS to be handled at parsing, not lexical analysis
    LeftAngle,

    #[token(">")] //FOR GENERICS to be handled at parsing, not lexical analysis
    RightAngle,

    //Special chars
    #[token("!")]
    ExclamationMark,

    #[token("?")]
    QuestionMark,

    #[token("$")]
    Dollarsign,
}

fn main() {
    let mut lex = Token::lexer("My mom said that 24 == 6 * 4");

    while let Some(token) = lex.next() {
        //Going through all tokens
        println!("{:?} : {:?}", token, lex.slice());
    }
}
