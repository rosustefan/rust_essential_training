// run using these (or other) CLI arguments:
// cargo run Moon 1969 --flag
// ... in order to get this output:
// argument 0 is target/debug/cli_args
// argument 1 is Moon
// argument 2 is 1969
// argument 3 is --flag

use std::env;

fn main() {
    if env::args().len() <= 2 {
        println!("Program requires at least 2 arguments.");
        return; // exit the program from the main() fn to terminate it
    }

    for (index, argument) in env::args().enumerate() {
        println!("argument {} is {}", index, argument);
    }

    let arg2 = env::args().nth(2).unwrap();
    println!("arg2 is {}", arg2);
}
