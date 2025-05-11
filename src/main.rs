use std::{env, fs};

use frontend::main::run;

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
    //let input: &str = "var int x = 5; var int y = 7; x = y; print(x);";
    //let input: &str = "var int x = 5; fn int f(int x) { var int x = 10; };";

    //Read file_name from command args
    let args: Vec<String> = env::args().collect();
    let debug_mode = args.contains(&"debug=true".to_string());
    if args.len() < 2 || (args.len() == 2 && debug_mode) {
        panic!("Usage: {} <file_name> [debug=true]", args[0]);
    }
    let file_name = &args[1];

    //Read file given as command arg
    match fs::read_to_string(file_name) {
        Ok(input) => {
            //Run wrench interpreter with file content as input
            run(&input, debug_mode);
        }
        Err(e) => {
            panic!("Error reading file: {}", e)
        }
    }
}
