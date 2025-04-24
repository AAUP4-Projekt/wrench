//Define enum

use logos::Logos;

#[derive(Logos, Debug, PartialEq, Clone)]
pub enum Token {
    //ignore whitespace
    #[regex(r"[ \t\n\f]+", logos::skip)]
    Whitespace,

    //ignore oneline comments like this one
    #[regex(r"//[^\n]*", logos::skip)]
    Comment,

    // Identifiers variables, or function names
    #[regex("[a-zA-Z_][a-zA-Z_]*", |lex| lex.slice().to_string())]
    Identifier(String),

    //Operators
    #[token("**")]
    Expon,

    #[token("*")]
    Star,

    #[token("==")]
    EqualsOperator,

    #[token("=")]
    AssignmentOperator,

    #[token("+")]
    Plus,

    #[token("%")]
    Modulo,

    #[token("-")]
    Minus,

    #[token("/")]
    Slash,

    #[token("or")]
    LogicalOr,

    #[token("and")]
    LogicalAnd,

    //Constants
    #[regex("[0-9]+", priority = 2, callback = parse_integer)] //Priority above identifiers
    Integer(i32),

    #[regex(r"[0-9]+\.[0-9]+", priority = 2, callback = parse_double)]
    Doubleliteral(f64),

    //Keywords
    #[token("bool")]
    Boolean,

    #[token("int")]
    IntegerKeyword,

    #[token("double")]
    DoubleKeyword,

    #[token("string")]
    String,

    #[token("table")]
    Table,

    #[token("row")]
    Row,

    #[token("pipe")]
    Pipe,

    #[token("rpipe")]
    Rpipe,

    #[token("fn")]
    Function,

    #[token("return")]
    Return,

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

    #[token("for")]
    For,

    //Literals
    #[regex(r#""([^"\\]|\\.)*""#, |lex| lex.slice().to_string())]
    Stringliteral(String),

    //Punctuators
    #[token(";")]
    Semicolon,

    #[token(",")]
    Comma,

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

fn parse_integer(lex: &mut logos::Lexer<Token>) -> i32 {
    lex.slice().parse().unwrap_or(0)
}

fn parse_double(lex: &mut logos::Lexer<Token>) -> f64 {
    lex.slice().parse().unwrap()
}
