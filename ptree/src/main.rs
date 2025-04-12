macro_rules! lalrpop_mod_doc {
    ($vis:vis $name:ident) => {
        lalrpop_util::lalrpop_mod!(
            #[allow(clippy::ptr_arg)]
            #[allow(clippy::vec_box)]
            $vis $name);
    }
}

lalrpop_mod_doc!(pub calculator4);
mod ast;

#[test]
fn calculator4() {
    let expr = calculator4::ExprParser::new()
        .parse("22 * 44 + 66")
        .unwrap();
}

#[cfg(not(test))]
fn main() {
    let parser = calculator4::ExprParser::new();
    let expr = "3 + 3 * 2 + 5 * (2 - 1)";  // Example expression

    match parser.parse(expr) {
        Ok(ast) => {
            println!("Parse Tree: {:#?}", ast);  // Pretty-print the parse tree
        }
        Err(e) => {
            eprintln!("Parse error: {}", e);
        }
    }
}
