//Define enum
use logos::Logos;

#[derive(Logos, Debug, PartialEq, Clone)]
pub enum Token {
    //ignore whitespace
    #[regex(r"[ \t\r\n\f]+", logos::skip)]
    Whitespace,

    //ignore oneline comments like this one
    #[regex(r"//[^\n]*", logos::skip)]
    Comment,

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

    #[token("for")]
    For,

    #[token("in")]
    In,

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

    #[token("<")]
    LeftAngle,

    #[token(">")]
    RightAngle,

    #[token("<=")]
    LessThanOrEqual,

    #[token(">=")]
    GreaterThanOrEqual,

    //Special chars
    #[token("!")]
    ExclamationMark,

    #[token(".")]
    Dot,

    // Identifiers variables, or function names
    #[regex("[a-zA-Z_][a-zA-Z_]*", |lex| lex.slice().to_string())]
    Identifier(String),

    //Literals
    #[regex(r#""([^"\\]|\\.)*""#, callback = parse_string)] //Things like "Hello"
    Stringliteral(String),
}

fn parse_integer(lex: &mut logos::Lexer<Token>) -> i32 {
    lex.slice().parse().unwrap()
}

fn parse_double(lex: &mut logos::Lexer<Token>) -> f64 {
    lex.slice().parse().unwrap()
}

fn parse_string(lex: &mut logos::Lexer<Token>) -> String {
    let content = lex.slice();
    content[1..content.len() - 1].to_string() // Strip the quotes
}

//Unit tests for lexer
#[cfg(test)]
mod tests {
    use super::*; //this is for importing names from outer scope
    use logos::Logos;

    //Careful! We return Result<Token
    #[test]
    fn test_for_integers_and_doubles() {
        let mut lexer = Token::lexer("5000 3.1415926535");

        assert_eq!(lexer.next(), Some(Ok(Token::Integer(5000))));
        assert_eq!(lexer.next(), Some(Ok(Token::Doubleliteral(3.1415926535))));
    }

    #[test]
    fn test_for_operators() {
        //We return Token
        let mut lexer = Token::lexer("** * / + - == = % and or");

        assert_eq!(lexer.next(), Some(Ok(Token::Expon)));
        assert_eq!(lexer.next(), Some(Ok(Token::Star)));
        assert_eq!(lexer.next(), Some(Ok(Token::Slash)));
        assert_eq!(lexer.next(), Some(Ok(Token::Plus)));
        assert_eq!(lexer.next(), Some(Ok(Token::Minus)));
        assert_eq!(lexer.next(), Some(Ok(Token::EqualsOperator)));
        assert_eq!(lexer.next(), Some(Ok(Token::AssignmentOperator)));
        assert_eq!(lexer.next(), Some(Ok(Token::Modulo)));
        assert_eq!(lexer.next(), Some(Ok(Token::LogicalAnd)));
        assert_eq!(lexer.next(), Some(Ok(Token::LogicalOr)));
    }

    #[test]
    fn test_for_specialchars() {
        let mut lexer = Token::lexer("!");
        assert_eq!(lexer.next(), Some(Ok(Token::ExclamationMark)));
    }

    #[test]
    fn test_for_keywords() {
        let mut lexer = Token::lexer(
            "bool int double string table row pipe fn return var const null true false if else while for",
        );

        assert_eq!(lexer.next(), Some(Ok(Token::Boolean)));
        assert_eq!(lexer.next(), Some(Ok(Token::IntegerKeyword)));
        assert_eq!(lexer.next(), Some(Ok(Token::DoubleKeyword)));
        assert_eq!(lexer.next(), Some(Ok(Token::String)));
        assert_eq!(lexer.next(), Some(Ok(Token::Table)));
        assert_eq!(lexer.next(), Some(Ok(Token::Row)));
        assert_eq!(lexer.next(), Some(Ok(Token::Pipe)));
        assert_eq!(lexer.next(), Some(Ok(Token::Function)));
        assert_eq!(lexer.next(), Some(Ok(Token::Return)));
        assert_eq!(lexer.next(), Some(Ok(Token::Var)));
        assert_eq!(lexer.next(), Some(Ok(Token::Constant)));
        assert_eq!(lexer.next(), Some(Ok(Token::Null)));
        assert_eq!(lexer.next(), Some(Ok(Token::True)));
        assert_eq!(lexer.next(), Some(Ok(Token::False)));
        assert_eq!(lexer.next(), Some(Ok(Token::If)));
        assert_eq!(lexer.next(), Some(Ok(Token::Else)));
        assert_eq!(lexer.next(), Some(Ok(Token::While)));
        assert_eq!(lexer.next(), Some(Ok(Token::For)));
    }

    #[test]
    fn test_for_punctuators() {
        let mut lexer = Token::lexer("; , ( ) { } [ ] < >");
        assert_eq!(lexer.next(), Some(Ok(Token::Semicolon)));
        assert_eq!(lexer.next(), Some(Ok(Token::Comma)));
        assert_eq!(lexer.next(), Some(Ok(Token::Openparan)));
        assert_eq!(lexer.next(), Some(Ok(Token::Closeparan)));
        assert_eq!(lexer.next(), Some(Ok(Token::Opencurlybracket)));
        assert_eq!(lexer.next(), Some(Ok(Token::Closecurlybracket)));
        assert_eq!(lexer.next(), Some(Ok(Token::Opensquarebracket)));
        assert_eq!(lexer.next(), Some(Ok(Token::Closesquarebracket)));
        assert_eq!(lexer.next(), Some(Ok(Token::LeftAngle)));
        assert_eq!(lexer.next(), Some(Ok(Token::RightAngle)));
    }

    #[test]
    fn test_for_whitespace() {
        let mut lexer = Token::lexer("                  ");
        assert_eq!(lexer.next(), None);
    }

    #[test]
    fn test_for_string_literals() {
        let mut lexer =
            Token::lexer("\"Hi, my name is Wrench! Pleased to make your acquaintance\"");

        assert_eq!(
            lexer.next(),
            Some(Ok(Token::Stringliteral(
                "Hi, my name is Wrench! Pleased to make your acquaintance".to_string()
            )))
        );
    }

    #[test]
    fn test_for_identifiers() {
        let mut lexer = Token::lexer("my_first_variable_name");
        assert_eq!(
            lexer.next(),
            Some(Ok(Token::Identifier("my_first_variable_name".to_string())))
        );
    }

    #[test]
    fn test_identifier_with_operator() {
        let mut lexer = Token::lexer("ident*ifier");
        assert_eq!(
            lexer.next(),
            Some(Ok(Token::Identifier("ident".to_string())))
        );
        assert_eq!(lexer.next(), Some(Ok(Token::Star)));
        assert_eq!(
            lexer.next(),
            Some(Ok(Token::Identifier("ifier".to_string())))
        );
    }
    #[test]
    fn invalid_input() {
        let mut lexer = Token::lexer("@ £ §");
        assert_eq!(lexer.next(), Some(Err(())));
        assert_eq!(lexer.next(), Some(Err(())));
        assert_eq!(lexer.next(), Some(Err(())));
    }
}
