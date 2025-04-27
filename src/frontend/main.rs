use super::ast::Statement;
use lalrpop_util::lalrpop_mod;
use logos::Logos;

use super::lexer::Token;

lalrpop_mod!(#[allow(clippy::all)] pub grammar);

//Lex tokens from input and parse them into a syntax tree
pub fn create_syntax_tree(input: &str) -> Vec<Statement> {
    //Lex
    let lexer = Token::lexer(input);

    //Collect tokens
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

    let parser = grammar::ProgramParser::new();

    match parser.parse(tokens) {
        Ok(program) => program,
        Err(e) => panic!("Parse error: {:?}", e),
    }
}

//Print the syntax tree for debugging purposes
fn print_syntax_tree(syntax_tree: &[Statement]) {
    for (i, statement) in syntax_tree.iter().enumerate() {
        println!("Statement {}: {:?}", i + 1, statement);
    }
}

//Create the AST from the input string
pub fn create_ast(input: &str) {
    // Create a syntax tree from the input string
    let syntax_tree = create_syntax_tree(input);
    print_syntax_tree(&syntax_tree);
    // TODO: Type check and return the AST
}

/*
    Unit Tests for parser
*/
#[cfg(test)]
mod tests {
    use super::super::ast::{Declaration, Expr, Misc, Operator, Statement, TypeConstruct};
    use super::*; // Import the module being tested // Import the AST types

    #[test]
    fn correct_expression_parse() {
        //Test if input parses correctly
        // Arrange
        let expected_syntax_tree = vec![Statement::Expr(Box::new(Expr::Operation(
            Box::new(Expr::Number(3)),
            Operator::Add,
            Box::new(Expr::Operation(
                Box::new(Expr::Number(5)),
                Operator::Mul,
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
            Operator::Add,
            Box::new(Expr::Operation(
                Box::new(Expr::Number(5)),
                Operator::Add, //Incorrect operator for the test
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
            Operator::Exp,
            Box::new(Expr::Operation(
                Box::new(Expr::Number(2)),
                Operator::Exp,
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
                Operator::Add,
                Box::new(Expr::Number(5)),
            )),
            Operator::Add,
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
                Operator::Add,
                Box::new(Expr::Number(5)),
            )),
            Operator::Mul,
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
        let expected_syntax_tree = vec![Statement::Declaration(Declaration::FunctionDeclaration(
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
        let expected_syntax_tree = vec![Statement::Declaration(Declaration::FunctionDeclaration(
            TypeConstruct::Int,
            "b".to_string(),
            vec![Misc::Parameter(TypeConstruct::Int, "x".to_string())],
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
}
