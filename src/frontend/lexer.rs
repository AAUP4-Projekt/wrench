use logos::Logos;
use std::fmt;

#[derive(Logos, Debug, PartialEq, Clone)]
pub enum Token {
    // Keywords
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

    // Symbols
    #[token("=")]
    AssignmentOperator,
    #[token("==")]
    EqualsOperator,
    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    #[token("*")]
    Star,
    #[token("/")]
    Slash,
    #[token(";")]
    Semicolon,
    #[token("!")]
    ExclamationMark,
    #[token("?")]
    QuestionMark,
    #[token("(")]
    LeftParen,
    #[token(")")]
    RightParen,

    // Identifiers and numbers
    #[regex("[a-zA-Z_][a-zA-Z_]*")]
    Identifier,
    #[regex(r"-?([0-9]+)", priority = 2, callback = parse_integer)]
    IntegerNumber(i32),


    // Whitespace
    #[regex(r"[ \t\n\f]+", logos::skip)] // Skip whitespace
    Whitespace,
}

fn parse_integer(lex: &mut logos::Lexer<Token>) -> i32 {
    lex.slice().parse().unwrap_or(0)
}

/*
// Implementing the Display trait for Token
// This allows us to print Token values using {} (e.g., println!("{}", token))
impl fmt::Display for Token {
    // The fmt method defines how to format the Token
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Match the current variant of the Token enum
        match self {
            // If the token is Plus, write "+"
            Token::Plus => write!(f, "+"),
            Token::Minus => write!(f, "-"),
            Token::Star => write!(f, "*"),
            Token::Slash => write!(f, "/"),
            Token::LeftParen => write!(f, "("),
            Token::RightParen => write!(f, ")"),
            // If the token is an integer number, write its value
            Token::IntegerNumber(n) => write!(f, "{}", n), 
            // For any other token, write "unknown token"
            _ => write!(f, "unknown token"),
        }
    }
}
*/

/*

pub fn print_tokens(tokens: &[(usize, Token, usize)]) {
    print!("Tokens: ");
    for (_, token, _) in tokens {
        print!("{:?} ", token);
    }
    println!();
}
*/