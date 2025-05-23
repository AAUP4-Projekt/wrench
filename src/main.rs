use std::{env, fs};

use frontend::main::run;

mod backend;
mod frontend;

//#[cfg(not(test))]
fn main() {
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