use frontend::frontend::create_ast;

mod frontend;
mod backend;


/*
Statement(
    Expr(
        Box::new(Op(
            Box::new(Number(3)), 
            +, 
            Box::new(Op(
                Box::new(Number(5)),
                 *, 
                Box::new(Op(
                    Box::new(Number(2)), 
                    *, 
                    Box::new(Number(3))
                ))
            ))
        ))
    )
)

Statement(
    Expr(
        Box::new(Op(
            Box::new(Number(2)), 
            +, 
            Box::new(Number(3))
        ))
    )
)
*/

//#[cfg(not(test))]
fn main() {
    let input = "3 + 5 * (2 * 3); 2 + 3;";
    create_ast(input);
}