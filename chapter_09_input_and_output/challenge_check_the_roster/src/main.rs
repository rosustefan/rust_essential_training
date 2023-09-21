use std::env;
use std::fs;

fn main() {
    if env::args().len() != 3 {
        println!("Program requires 2 arguments: an input file and an astronaut name.");
        return;
    }

    let input_file = env::args().nth(1).unwrap();
    let astronaut_name = env::args().nth(2).unwrap();

    let input_file_contents = fs::read_to_string(input_file.clone()).unwrap();
    for line in input_file_contents.lines() {
        // println!("{}", astronaut);
        if astronaut_name == line {
            println!(
                "Found the astronaut {} in the {} file!",
                astronaut_name, input_file
            );
            return;
        }
    }

    println!(
        "Did not find the astronaut '{}' in the '{}' file.",
        astronaut_name, input_file
    );
}
