use logos::Logos;
use std::fmt;

#[derive(Logos, Debug, PartialEq, Clone)]
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

    #[token("{")]
    Opencurlybracket,

    #[token("}")]
    Closecurlybracket,

    #[token("[")]
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
    let mut lex = Token::lexer("My mom said that 24 = 6 * 4");

    while let Some(token) = lex.next() {
        //Going through all tokens
        println!("{:?} : {:?}", token, lex.slice());
    }
}


pub fn print_tokens(tokens: &[(usize, Token, usize)]) {
    print!("Tokens: ");
    for (_, token, _) in tokens {
        print!("{:?} ", token);
    }
    println!();
}


/*
pub fn collect_tokens(lexer: Token::Lexer) -> Vec<(usize, Token, usize)> {
    lexer
        .spanned()
        .filter_map(|(token, span)| match token {
            Ok(t) => Some((span.start, t, span.end)),
            Err(_) => {
                eprintln!("Invalid token at {:?}", span);
                None
            },
        })
        .collect()
}
*/

fn parse_integer(lex: &mut logos::Lexer<Token>) -> i32 {
    lex.slice().parse().unwrap_or(0)
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::Plus => write!(f, "+"),
            Token::Minus => write!(f, "-"),
            Token::Star => write!(f, "*"),
            Token::Slash => write!(f, "/"),
            Token::LeftParen => write!(f, "("),
            Token::RightParen => write!(f, ")"),
            Token::IntegerNumber(n) => write!(f, "{}", n), 
            _ => write!(f, "unknown token"),
        }
    }
}
