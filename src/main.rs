use frontend::main::create_ast;

mod backend;
mod frontend;

//#[cfg(not(test))]
fn main() {
    let input = "int x = 7; x = 5; //Hello\n int y = (3 ** 2 ** 1);";
    println!("Program: {}\n\nParsing:", input);
    create_ast(input);
}
