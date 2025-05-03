use frontend::main::create_ast;

mod backend;
mod frontend;

//#[cfg(not(test))]
fn main() {
    //let input = "var int x = 7; x = 5; //Hello\n var int y = (3 ** 2 ** 1);";
    //let input = "fn int b(int b, int y){var int x = 3; var <x>[] b = false;}; return;";
    //let input = "var fn int (int, double) x = 4;";
    //let input = "4 pipe x() pipe y(x); x(3, 5, true);";
    //let input = "print(x);";
    //let input = "var int x = 5; fn int b(int b, int y){var int y = 3;}; y = 3.3;";
    let input = "var int x = 5; fn int f(int x) { var int x = 10; };";
    println!("Program: {}\n\nParsing:", input);
    create_ast(input);
}
