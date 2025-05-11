use crate::backend::evaluate::interpret;

use super::ast::Statement;
use lalrpop_util::{ParseError, lalrpop_mod};
use logos::Logos;

use super::lexer::Token;
//use super::typecheck::type_check;

lalrpop_mod!(#[allow(clippy::all)] pub grammar);

fn lex(input: &str) -> Vec<(usize, Token, usize)> {
    let lexer = Token::lexer(input);
    let tokens: Vec<_> = lexer
        .spanned()
        .filter_map(|(token, span)| match token {
            Ok(t) => Some((span.start, t, span.end)),
            Err(_) => {
                eprintln!("Invalid token at {:?}", span);
                None
            }
        })
        .collect();
    tokens
}

fn parse(tokens: Vec<(usize, Token, usize)>) -> Statement {
    let parser = grammar::ProgramParser::new();
    match parser.parse(tokens) {
        Ok(program) => program,
        Err(e) => {
            match e {
                ParseError::InvalidToken { location } => {
                    eprintln!("Invalid token at position {}", location);
                }
                ParseError::UnrecognizedToken { token, expected } => {
                    let (start, token, end) = token;
                    eprintln!(
                        "Unrecognized token {:?} at position {}-{}. Expected one of: {:?}",
                        token, start, end, expected
                    );
                }
                ParseError::ExtraToken { token } => {
                    let (start, token, end) = token;
                    eprintln!("Extra token {:?} at position {}-{}", token, start, end);
                }
                ParseError::User { error } => {
                    eprintln!("Custom error: {}", error);
                }
                ParseError::UnrecognizedEof { location, expected } => {
                    eprintln!(
                        "Unrecognized EOF at position {}. Expected one of: {:?}",
                        location, expected
                    );
                }
            }
            panic!();
        }
    }
}

//Lex tokens from input and parse them into a syntax tree
pub fn create_syntax_tree(input: &str) -> Statement {
    //Collect tokens
    let tokens = lex(input);
    //Parse tokens and return the syntax tree
    parse(tokens)
}

//Create the AST from the input string
pub fn run(input: &str, debug_mode: bool) {
    // Opret syntakstræ fra input
    let syntax_tree = create_syntax_tree(input);
    // Print syntaxtree
    if debug_mode {
        println!("Syntaxtree: {:?}", syntax_tree);
    }

    interpret(syntax_tree);

    /*
    match type_check(&syntax_tree) {
        Ok(typed_syntax_tree) => {
            println!("Type checking passed!");
            print_syntax_tree(&typed_syntax_tree);
            println!("Interpreting:");
            interpret(typed_syntax_tree);
        }
        Err(e) => {
            eprintln!("Type checking failed: {}", e);
        }
    }
    */
}

/*
========================================================
Unit Tests for parser
========================================================
*/
#[cfg(test)]
mod tests {
    use super::super::ast::{
        ColumnAssignmentEnum, Declaration, Expr, Operator, Parameter, Statement, TypeConstruct,
    };
    use super::super::lexer::Token; // Import the Token enum from the lexer module
    use super::{create_syntax_tree, parse}; // Import the module being tested // Import the AST types

    // Helper function for create a tuple of (usize, Token, usize)
    fn f(t: Token) -> (usize, Token, usize) {
        return (0, t, 0);
    }

    #[test]
    fn tokens_are_pased_1() {
        // Arrange
        let tokens = vec![
            f(Token::Integer(3)),
            f(Token::Plus),
            f(Token::Integer(5)),
            f(Token::Star),
            f(Token::Integer(2)),
            f(Token::Semicolon),
        ];

        let expected_syntax_tree = vec![Statement::Expr(Box::new(Expr::Operation(
            Box::new(Expr::Number(3)),
            Operator::Addition,
            Box::new(Expr::Operation(
                Box::new(Expr::Number(5)),
                Operator::Multiplication,
                Box::new(Expr::Number(2)),
            )),
        )))];

        // Act
        let syntax_tree = parse(tokens);

        // Assert
        assert_eq!(syntax_tree, expected_syntax_tree);
    }

    #[test]
    fn tokens_are_pased_2() {
        // Arrange
        let tokens = vec![
            f(Token::Table),
            f(Token::Openparan),
            f(Token::IntegerKeyword),
            f(Token::Identifier("id".to_string())),
            f(Token::Comma),
            f(Token::String),
            f(Token::Identifier("name".to_string())),
            f(Token::Closeparan),
            f(Token::Semicolon),
            f(Token::ExclamationMark),
            f(Token::True),
            f(Token::Semicolon),
        ];

        let expected_syntax_tree = vec![
            Statement::Expr(Box::new(Expr::Table(vec![
                Parameter::Parameter(TypeConstruct::Int, "id".to_string()),
                Parameter::Parameter(TypeConstruct::String, "name".to_string()),
            ]))),
            Statement::Expr(Box::new(Expr::Not(Box::new(Expr::Bool(true))))),
        ];

        // Act
        let syntax_tree = parse(tokens);

        // Assert
        assert_eq!(syntax_tree, expected_syntax_tree);
    }

    /*
    ========================================================
    Integration Tests for parser
    ========================================================
    */

    #[test]
    fn correct_expression_parse() {
        //Test if input parses correctly
        // Arrange
        let expected_syntax_tree = vec![Statement::Expr(Box::new(Expr::Operation(
            Box::new(Expr::Number(3)),
            Operator::Addition,
            Box::new(Expr::Operation(
                Box::new(Expr::Number(5)),
                Operator::Multiplication,
                Box::new(Expr::Number(2)),
            )),
        )))];

        // Act
        let syntax_tree = create_syntax_tree("3 + 5 * 2;");

        //Assert
        assert_eq!(syntax_tree, expected_syntax_tree);
    }

    #[test]
    fn incorrect_expression_parse() {
        //Test if wrong input parses incorrectly
        // Arrange
        let expected_syntax_tree = vec![Statement::Expr(Box::new(Expr::Operation(
            Box::new(Expr::Number(3)),
            Operator::Addition,
            Box::new(Expr::Operation(
                Box::new(Expr::Number(5)),
                Operator::Addition, //Incorrect operator for the test
                Box::new(Expr::Number(2)),
            )),
        )))];

        // Act
        let syntax_tree = create_syntax_tree("3 + 5 * 2;");

        //Assert
        assert_ne!(syntax_tree, expected_syntax_tree);
    }

    #[test]
    fn comments_and_witespace_ignored() {
        //Test if comments and whitespace are ignored
        // Arrange
        let expected_syntax_tree = vec![
            Statement::Expr(Box::new(Expr::Number(3))),
            Statement::Expr(Box::new(Expr::Number(2))),
        ];

        // Act
        let syntax_tree = create_syntax_tree("3;      //Comment ag \n2;");

        //Assert
        assert_eq!(syntax_tree, expected_syntax_tree);
    }

    #[test]
    fn exponent_right_to_left_associativity() {
        //Test if exponentiation is right associative
        // Arrange
        let expected_syntax_tree = vec![Statement::Expr(Box::new(Expr::Operation(
            Box::new(Expr::Number(3)),
            Operator::Exponent,
            Box::new(Expr::Operation(
                Box::new(Expr::Number(2)),
                Operator::Exponent,
                Box::new(Expr::Number(1)),
            )),
        )))];

        // Act
        let syntax_tree = create_syntax_tree("3 ** 2 ** 1;");

        //Assert
        assert_eq!(syntax_tree, expected_syntax_tree);
    }

    #[test]
    fn addition_left_to_right_associativity() {
        //Test if addition is left associative
        // Arrange
        let expected_syntax_tree = vec![Statement::Expr(Box::new(Expr::Operation(
            Box::new(Expr::Operation(
                Box::new(Expr::Number(3)),
                Operator::Addition,
                Box::new(Expr::Number(5)),
            )),
            Operator::Addition,
            Box::new(Expr::Number(2)),
        )))];

        // Act
        let syntax_tree = create_syntax_tree("3 + 5 + 2;");

        //Assert
        assert_eq!(syntax_tree, expected_syntax_tree);
    }

    #[test]
    fn parenteses_have_high_presedence() {
        //Test if parentheses have higher precedence than multiplication
        // Arrange
        let expected_syntax_tree = vec![Statement::Expr(Box::new(Expr::Operation(
            Box::new(Expr::Operation(
                Box::new(Expr::Number(3)),
                Operator::Addition,
                Box::new(Expr::Number(5)),
            )),
            Operator::Multiplication,
            Box::new(Expr::Number(2)),
        )))];

        // Act
        let syntax_tree = create_syntax_tree("(3 + 5) * 2;");

        //Assert
        assert_eq!(syntax_tree, expected_syntax_tree);
    }

    #[test]
    fn parses_empty_functions() {
        //Test if empty functions are parsed correctly
        // Arrange
        let expected_syntax_tree = vec![Statement::Declaration(Declaration::Function(
            TypeConstruct::Int,
            "b".to_string(),
            vec![],
            vec![],
        ))];

        // Act
        let syntax_tree = create_syntax_tree("fn int b(){};");

        //Assert
        assert_eq!(syntax_tree, expected_syntax_tree);
    }

    #[test]
    fn parses_function_with_parameters_and_body() {
        //Test if functions with parameters are parsed correctly
        // Arrange
        let expected_syntax_tree = vec![Statement::Declaration(Declaration::Function(
            TypeConstruct::Int,
            "b".to_string(),
            vec![Parameter::Parameter(TypeConstruct::Int, "x".to_string())],
            vec![Statement::VariableAssignment(
                "x".to_string(),
                Box::new(Expr::Number(3)),
            )],
        ))];

        // Act
        let syntax_tree = create_syntax_tree("fn int b(int x){x = 3;};");

        // Assert
        assert_eq!(syntax_tree, expected_syntax_tree);
    }

    #[test]
    fn parses_tables_and_rows() {
        // Test if tables and rows are parsed correctly
        // Arrange
        let expected_syntax_tree = vec![
            Statement::Expr(Box::new(Expr::Table(vec![
                Parameter::Parameter(TypeConstruct::Int, "id".to_string()),
                Parameter::Parameter(TypeConstruct::String, "name".to_string()),
            ]))),
            Statement::Expr(Box::new(Expr::Row(vec![
                ColumnAssignmentEnum::ColumnAssignment(
                    TypeConstruct::Int,
                    "id".to_string(),
                    Box::new(Expr::Number(1)),
                ),
                ColumnAssignmentEnum::ColumnAssignment(
                    TypeConstruct::String,
                    "name".to_string(),
                    Box::new(Expr::Identifier("Alice".to_string())),
                ),
            ]))),
        ];

        // Act
        let syntax_tree =
            create_syntax_tree("table(int id, string name); row(int id = 1, string name = Alice);");

        // Assert
        assert_eq!(syntax_tree, expected_syntax_tree);
    }

    #[test]
    fn parses_boolean_operators() {
        // Test if boolean operators are parsed correctly
        // Arrange
        let expected_syntax_tree = vec![Statement::Expr(Box::new(Expr::Operation(
            Box::new(Expr::Operation(
                Box::new(Expr::Bool(true)),
                Operator::And,
                Box::new(Expr::Bool(false)),
            )),
            Operator::Or,
            Box::new(Expr::Bool(true)),
        )))];

        // Act
        let syntax_tree = create_syntax_tree("true and false or true;");

        // Assert
        assert_eq!(syntax_tree, expected_syntax_tree);
    }

    #[test]
    fn parses_doubles() {
        // Test if double literals are parsed correctly
        // Arrange
        let expected_syntax_tree = vec![Statement::Expr(Box::new(Expr::Double(3.14)))];

        // Act
        let syntax_tree = create_syntax_tree("3.14;");

        // Assert
        assert_eq!(syntax_tree, expected_syntax_tree);
    }

    #[test]
    fn parses_null() {
        // Test if null values are parsed correctly
        // Arrange
        let expected_syntax_tree = vec![Statement::Expr(Box::new(Expr::Null))];

        // Act
        let syntax_tree = create_syntax_tree("null;");

        // Assert
        assert_eq!(syntax_tree, expected_syntax_tree);
    }

    #[test]
    fn parses_double_negation() {
        // Test if double negation is parsed correctly
        // Arrange
        let expected_syntax_tree = vec![Statement::Expr(Box::new(Expr::Not(Box::new(Expr::Not(
            Box::new(Expr::Bool(true)),
        )))))];

        // Act
        let syntax_tree = create_syntax_tree("!!true;");

        // Assert
        assert_eq!(syntax_tree, expected_syntax_tree);
    }
}
