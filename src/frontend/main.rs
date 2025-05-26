use std::collections::HashMap;

use crate::backend::evaluate::interpret;

use super::{
    ast::{Statement, TypeConstruct},
    typecheck::{VariableInfo, type_check},
};
use lalrpop_util::{ParseError, lalrpop_mod};
use logos::Logos;

use super::lexer::Token;

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
        Err(e) => match e {
            ParseError::InvalidToken { location } => {
                panic!("Invalid token at position {}", location);
            }
            ParseError::UnrecognizedToken { token, expected } => {
                let (start, token, end) = token;
                panic!(
                    "Unrecognized token {:?} at position {}-{}. Expected one of: {:?}",
                    token, start, end, expected
                );
            }
            ParseError::ExtraToken { token } => {
                let (start, token, end) = token;
                panic!("Extra token {:?} at position {}-{}", token, start, end);
            }
            ParseError::User { error } => {
                panic!("Custom error: {}", error);
            }
            ParseError::UnrecognizedEof { location, expected } => {
                if expected.contains(&"\";\"".to_string()) {
                    panic!("Parse error : Missing semicolon at the end of the declaration!")
                } else {
                    panic!(
                        "Unrecognized EOF at position {}. Expected one of: {:?}",
                        location, expected
                    );
                }
            }
        },
    }
}

// Define a global environment for functions
fn create_global_environment() -> HashMap<String, VariableInfo> {
    let mut global_env = HashMap::new();

    // print: (any) -> table
    global_env.insert(
        "print".to_string(),
        VariableInfo {
            var_type: TypeConstruct::Function(
                Box::new(TypeConstruct::Table(vec![])),
                vec![TypeConstruct::Any],
            ),
            is_constant: false,
        },
    );

    // import: (string, table) -> table
    global_env.insert(
        "import".to_string(),
        VariableInfo {
            var_type: TypeConstruct::Function(
                Box::new(TypeConstruct::Table(vec![])),
                vec![TypeConstruct::String, TypeConstruct::Table(vec![])],
            ),
            is_constant: false,
        },
    );
    // async_import: (string, table) -> table
    global_env.insert(
        "async_import".to_string(),
        VariableInfo {
            var_type: TypeConstruct::Function(
                Box::new(TypeConstruct::Table(vec![])),
                vec![TypeConstruct::String, TypeConstruct::Any],
            ),
            is_constant: false,
        },
    );

    // table_add_row: (table, row) -> null
    global_env.insert(
        "table_add_row".to_string(),
        VariableInfo {
            var_type: TypeConstruct::Function(
                Box::new(TypeConstruct::Null),
                vec![TypeConstruct::Any, TypeConstruct::Any],
            ),
            is_constant: false,
        },
    );

    global_env
}

//Lex tokens from input and parse them into a syntax tree
//pub fn create_syntax_tree(input: &str) -> Vec<Statement> {
pub fn create_syntax_tree(input: &str) -> Statement {
    ////Statement
    //Collect tokens
    let tokens: Vec<(usize, Token, usize)> = lex(input);
    //Parse tokens and return the syntax tree
    parse(tokens)
}

//Create the AST from the input string
pub fn run(input: &str, debug_mode: bool) {
    if debug_mode {
        println!("Input program:\n{}\n", input);
    }
    // Opret syntakstræ fra input
    let syntax_tree = create_syntax_tree(input);
    // Print syntaxtree
    if debug_mode {
        println!("Syntaxtree:\n{:?}\n", syntax_tree);
        println!("Evaluating:");
    }

    // Create a global environment for functions
    let global_env: HashMap<String, VariableInfo> = create_global_environment();

    // This stack of scopes keeps track of variable names and their types
    let mut scope_stack: Vec<HashMap<String, VariableInfo>> = vec![global_env];
    match type_check(&syntax_tree, &mut scope_stack) {
        Ok(_) => {
            interpret(syntax_tree);
        }
        Err(e) => {
            eprintln!("Type checking failed: {}", e);
        }
    }
}

/*
========================================================
Unit Tests for parser
========================================================
*/
#[cfg(test)]
mod tests {
    use super::super::ast::make_compound;
    use super::super::ast::{
        ColumnAssignmentEnum, Declaration, Expr, Operator, Parameter, Statement, TypeConstruct,
        ast_and,
    };
    use super::super::lexer::Token; // Import the Token enum from the lexer module
    use super::{create_syntax_tree, parse}; // Import the module being tested // Import the AST types

    // Helper function for create a tuple of (usize, Token, usize)
    fn f(t: Token) -> (usize, Token, usize) {
        return (0, t, 0);
    }

    #[test]
    fn tokens_are_parsed_1() {
        // Arrange
        let tokens = vec![
            f(Token::Integer(3)),
            f(Token::Plus),
            f(Token::Integer(5)),
            f(Token::Star),
            f(Token::Integer(2)),
            f(Token::Semicolon),
        ];

        let expected_syntax_tree =
            *make_compound(vec![Statement::Expr(Box::new(Expr::Operation(
                Box::new(Expr::Number(3)),
                Operator::Addition,
                Box::new(Expr::Operation(
                    Box::new(Expr::Number(5)),
                    Operator::Multiplication,
                    Box::new(Expr::Number(2)),
                )),
            )))]);

        // Act
        let syntax_tree = parse(tokens);

        // Assert
        assert_eq!(syntax_tree, expected_syntax_tree);
    }

    #[test]
    fn tokens_are_parsed_2() {
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

        let expected_syntax_tree = *make_compound(vec![
            Statement::Expr(Box::new(Expr::Table(vec![
                Parameter::Parameter(TypeConstruct::Int, "id".to_string()),
                Parameter::Parameter(TypeConstruct::String, "name".to_string()),
            ]))),
            Statement::Expr(Box::new(Expr::Not(Box::new(Expr::Bool(true))))),
        ]);

        // Act
        let syntax_tree = parse(tokens);

        // Assert
        assert_eq!(syntax_tree, expected_syntax_tree);
    }

    #[test] //testing in isolation
    fn test_addition_ast() {
        let expr = Expr::Operation(
            Box::new(Expr::Number(2)),
            Operator::Addition,
            Box::new(Expr::Number(2)),
        );
        assert_eq!(
            expr,
            Expr::Operation(
                Box::new(Expr::Number(2)),
                Operator::Addition,
                Box::new(Expr::Number(2)),
            )
        )
    }

    #[test]
    fn test_composition_statements() {
        let statements = vec![
            Statement::Expr(Box::new(Expr::Bool(true))),
            Statement::Expr(Box::new(Expr::Number(32))),
        ];
        let composition = make_compound(statements);

        let expected_ast = Box::new(Statement::Compound(
            Box::new(Statement::Expr(Box::new(Expr::Bool(true)))),
            Box::new(Statement::Compound(
                Box::new(Statement::Expr(Box::new(Expr::Number(32)))),
                Box::new(Statement::Skip),
            )),
        ));

        assert_eq!(composition, expected_ast);
    }

    #[test]
    fn test_logical_operators() {
        let leftside = Box::new(Expr::Bool(true));
        let rightside = Box::new(Expr::Bool(false));

        let and_expr = ast_and(leftside.clone(), rightside.clone());

        let expected_ast = Box::new(Expr::Not(Box::new(Expr::Operation(
            Box::new(Expr::Not(leftside)),
            Operator::Or,
            Box::new(Expr::Not(rightside)),
        ))));
        assert_eq!(and_expr, expected_ast)
    }

    #[test]
    fn test_parse_if_else() {
        let expected_syntax_tree = Statement::Compound(
            Box::new(Statement::If(
                Box::new(Expr::Bool(true)),
                Box::new(Statement::Compound(
                    Box::new(Statement::VariableAssignment(
                        "x".to_string(),
                        Box::new(Expr::Number(1)),
                    )),
                    Box::new(Statement::Skip),
                )),
                Box::new(Statement::Compound(
                    Box::new(Statement::VariableAssignment(
                        "x".to_string(),
                        Box::new(Expr::Number(0)),
                    )),
                    Box::new(Statement::Skip),
                )),
            )),
            Box::new(Statement::Skip),
        );

        let actual_syntax_tree = create_syntax_tree("if (true) { x = 1; } else { x = 0; }");

        assert_eq!(actual_syntax_tree, expected_syntax_tree);
    }

    #[test]
    fn test_while_loop() {
        let expected_ast = Statement::Compound(
            Box::new(Statement::While(
                Box::new(Expr::Bool(true)),
                Box::new(Statement::Compound(
                    Box::new(Statement::VariableAssignment(
                        "x".to_string(),
                        Box::new(Expr::Number(1)),
                    )),
                    Box::new(Statement::Skip),
                )),
            )),
            Box::new(Statement::Skip),
        );

        let actual_ast = create_syntax_tree("while (true) { x = 1; }");

        assert_eq!(actual_ast, expected_ast);
    }

    //Edge cases
    #[test]
    #[should_panic(expected = "Unrecognized token Closeparan")]
    fn unmatched_paran() {
        create_syntax_tree("100 + (2 * 3));");
    }

    #[test]
    #[should_panic(expected = "Unrecognized token")]
    fn unmatched_paran2() {
        create_syntax_tree("100 + (2 * 3;");
    }

    #[test]
    #[should_panic(expected = "Parse error : Missing semicolon at the end of the declaration!")]
    fn missing_semicolon() {
        create_syntax_tree("var int x = 2");
    }

    #[test]
    #[should_panic]
    fn invalid_identifiername() {
        create_syntax_tree("var ?myname = \"Isabella\""); //Illegal ident
    }

    #[test]
    #[should_panic]
    fn invalid_coma() {
        create_syntax_tree("print(100, 800, )"); //Illegal comma
    }
    #[test]
    #[should_panic]
    fn invalid_questionmark() {
        create_syntax_tree("print(100, 800? )"); //Illegal symbol
    }

    #[test]
    #[should_panic]
    fn nobody_function_declr() {
        create_syntax_tree("fn double dummy(double y);"); //Function has no body
    }

    #[test]
    #[should_panic]
    fn invalid_expr() {
        create_syntax_tree("11 + ??"); //Invalid operation.
    }

    #[test]
    #[should_panic]
    fn invalid_array_index() {
        create_syntax_tree("arr[0;");
    }

    #[test]
    #[should_panic]
    fn invalid_pipe_fnname() {
        create_syntax_tree("data pipe (0, 1); "); //Missing function name for pipe
    }

    #[test]
    #[should_panic]
    fn invalid_operation() {
        create_syntax_tree("1 ++ 2;"); //What is ++?
    }

    #[test]
    #[should_panic]
    fn invalid_row_decl() {
        create_syntax_tree("row(int age, string name);"); //Remember: we declare rows like row(int age = 5)
    }

    #[test]
    #[should_panic]
    fn invalid_table_decl() {
        create_syntax_tree("table(age, string name);"); //Missing the age type!
    }

    #[test]
    #[should_panic]
    fn no_statement() {
        create_syntax_tree(";"); //Empty statement should not be allowed
    }

    #[test]
    #[should_panic]
    fn callingfunction_incorrectly() {
        create_syntax_tree("myfunction(name age)"); //Dont forget commas between args
    }

    //Check that the correct version of edge cases is working!
    #[test]
    fn unmatched_paran_correct() {
        create_syntax_tree("100 + (2 * 3);");
    }

    #[test]
    fn unmatched_paran2_correct() {
        create_syntax_tree("100 + (2 * 3);");
    }

    #[test]
    fn missing_semicolon_correct() {
        create_syntax_tree("var int x = 2;");
    }

    #[test]
    fn invalid_identifiername_correct() {
        create_syntax_tree("var string myname = \"Isabella\";");
    }

    #[test]
    fn invalid_coma_and_questionmark_correct() {
        create_syntax_tree("print(100, 800 );");
    }

    #[test]
    fn nobody_function_declr_correct() {
        create_syntax_tree("fn double dummy(double y){};");
    }

    #[test]
    fn invalid_expr_correct() {
        create_syntax_tree("print(11 + 11);");
    }

    #[test]
    fn invalid_array_index_correct() {
        create_syntax_tree("arr[0];");
    }

    #[test]
    fn invalid_operation_correct() {
        create_syntax_tree("1 + 2;");
    }

    #[test]
    fn invalid_row_decl_correct() {
        create_syntax_tree("row(int age = 5);");
    }

    #[test]
    fn invalid_table_decl_correct() {
        create_syntax_tree("table(int age, string name);");
    }

    #[test]
    fn callingfunction_incorrectly_correct() {
        create_syntax_tree("myfunction(name , age);"); //Dont forget commas between args
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
        let expected_syntax_tree =
            *make_compound(vec![Statement::Expr(Box::new(Expr::Operation(
                Box::new(Expr::Number(3)),
                Operator::Addition,
                Box::new(Expr::Operation(
                    Box::new(Expr::Number(5)),
                    Operator::Multiplication,
                    Box::new(Expr::Number(2)),
                )),
            )))]);

        // Act
        let syntax_tree = create_syntax_tree("3 + 5 * 2;");

        //Assert
        assert_eq!(syntax_tree, expected_syntax_tree);
    }

    #[test]
    fn incorrect_expression_parse() {
        //Test if wrong input parses incorrectly
        // Arrange
        let expected_syntax_tree =
            *make_compound(vec![Statement::Expr(Box::new(Expr::Operation(
                Box::new(Expr::Number(3)),
                Operator::Addition,
                Box::new(Expr::Operation(
                    Box::new(Expr::Number(5)),
                    Operator::Addition, //Incorrect operator for the test
                    Box::new(Expr::Number(2)),
                )),
            )))]);

        // Act
        let syntax_tree = create_syntax_tree("3 + 5 * 2;");

        //Assert
        assert_ne!(syntax_tree, expected_syntax_tree);
    }

    #[test]
    fn comments_and_witespace_ignored() {
        //Test if comments and whitespace are ignored
        // Arrange
        let expected_syntax_tree = *make_compound(vec![
            Statement::Expr(Box::new(Expr::Number(3))),
            Statement::Expr(Box::new(Expr::Number(2))),
        ]);

        // Act
        let syntax_tree = create_syntax_tree("3;      //Comment ag \n2;");

        //Assert
        assert_eq!(syntax_tree, expected_syntax_tree);
    }

    #[test]
    fn exponent_right_to_left_associativity() {
        //Test if exponentiation is right associative
        // Arrange
        let expected_syntax_tree =
            *make_compound(vec![Statement::Expr(Box::new(Expr::Operation(
                Box::new(Expr::Number(3)),
                Operator::Exponent,
                Box::new(Expr::Operation(
                    Box::new(Expr::Number(2)),
                    Operator::Exponent,
                    Box::new(Expr::Number(1)),
                )),
            )))]);

        // Act
        let syntax_tree = create_syntax_tree("3 ** 2 ** 1;");

        //Assert
        assert_eq!(syntax_tree, expected_syntax_tree);
    }

    #[test]
    fn addition_left_to_right_associativity() {
        //Test if addition is left associative
        // Arrange
        let expected_syntax_tree =
            *make_compound(vec![Statement::Expr(Box::new(Expr::Operation(
                Box::new(Expr::Operation(
                    Box::new(Expr::Number(3)),
                    Operator::Addition,
                    Box::new(Expr::Number(5)),
                )),
                Operator::Addition,
                Box::new(Expr::Number(2)),
            )))]);

        // Act
        let syntax_tree = create_syntax_tree("3 + 5 + 2;");

        //Assert
        assert_eq!(syntax_tree, expected_syntax_tree);
    }

    #[test]
    fn parenteses_have_high_presedence() {
        //Test if parentheses have higher precedence than multiplication
        // Arrange
        let expected_syntax_tree =
            *make_compound(vec![Statement::Expr(Box::new(Expr::Operation(
                Box::new(Expr::Operation(
                    Box::new(Expr::Number(3)),
                    Operator::Addition,
                    Box::new(Expr::Number(5)),
                )),
                Operator::Multiplication,
                Box::new(Expr::Number(2)),
            )))]);

        // Act
        let syntax_tree = create_syntax_tree("(3 + 5) * 2;");

        //Assert
        assert_eq!(syntax_tree, expected_syntax_tree);
    }

    #[test]
    fn parses_empty_functions() {
        //Test if empty functions are parsed correctly
        // Arrange
        let expected_syntax_tree =
            *make_compound(vec![Statement::Declaration(Declaration::Function(
                TypeConstruct::Int,
                "b".to_string(),
                vec![],
                make_compound(vec![]),
            ))]);

        // Act
        let syntax_tree = create_syntax_tree("fn int b(){};");

        //Assert
        assert_eq!(syntax_tree, expected_syntax_tree);
    }

    #[test]
    fn parses_function_with_parameters_and_body() {
        //Test if functions with parameters are parsed correctly
        // Arrange
        let expected_syntax_tree =
            *make_compound(vec![Statement::Declaration(Declaration::Function(
                TypeConstruct::Int,
                "b".to_string(),
                vec![Parameter::Parameter(TypeConstruct::Int, "x".to_string())],
                make_compound(vec![Statement::VariableAssignment(
                    "x".to_string(),
                    Box::new(Expr::Number(3)),
                )]),
            ))]);

        // Act
        let syntax_tree = create_syntax_tree("fn int b(int x){x = 3;};");

        // Assert
        assert_eq!(syntax_tree, expected_syntax_tree);
    }

    #[test]
    fn parses_tables_and_rows() {
        // Test if tables and rows are parsed correctly
        // Arrange
        let expected_syntax_tree = *make_compound(vec![
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
        ]);

        // Act
        let syntax_tree =
            create_syntax_tree("table(int id, string name); row(int id = 1, string name = Alice);");

        // Assert
        assert_eq!(syntax_tree, expected_syntax_tree);
    }

    #[test]
    fn parses_boolean_operators() {
        let expected_syntax_tree =
            *make_compound(vec![Statement::Expr(Box::new(Expr::Operation(
                ast_and(Box::new(Expr::Bool(true)), Box::new(Expr::Bool(false))),
                Operator::Or,
                Box::new(Expr::Bool(true)),
            )))]);

        let syntax_tree = create_syntax_tree("true and false or true;");

        assert_eq!(syntax_tree, expected_syntax_tree);
    }

    #[test]
    fn parses_doubles() {
        // Test if double literals are parsed correctly
        // Arrange
        let expected_syntax_tree =
            *make_compound(vec![Statement::Expr(Box::new(Expr::Double(3.14)))]);

        // Act
        let syntax_tree = create_syntax_tree("3.14;");

        // Assert
        assert_eq!(syntax_tree, expected_syntax_tree);
    }

    #[test]
    fn parses_null() {
        // Test if null values are parsed correctly
        // Arrange
        let expected_syntax_tree = *make_compound(vec![Statement::Expr(Box::new(Expr::Null))]);

        // Act
        let syntax_tree = create_syntax_tree("null;");

        // Assert
        assert_eq!(syntax_tree, expected_syntax_tree);
    }

    #[test]
    fn parses_double_negation() {
        // Test if double negation is parsed correctly
        // Arrange
        let expected_syntax_tree = *make_compound(vec![Statement::Expr(Box::new(Expr::Not(
            Box::new(Expr::Not(Box::new(Expr::Bool(true)))),
        )))]);

        // Act
        let syntax_tree = create_syntax_tree("!!true;");

        // Assert
        assert_eq!(syntax_tree, expected_syntax_tree);
    }
}
