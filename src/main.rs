use frontend::frontend::create_ast;

mod frontend;
mod backend;


/*
let input = "3 + 5 * (2 * 3); 2 + 3;";

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
    let input = "int x = 7; bool y = x;";
    create_ast(input);
}