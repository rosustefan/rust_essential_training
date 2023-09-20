use std::fs;

fn main() {
    // read file to string
    let contents = fs::read_to_string("data/planets.txt").unwrap();
    println!("contents is {}", contents);

    for line in contents.lines() {
        println!("line is {}", line);
    }

    // read file to a vector / binary array of numbers
    let contents = fs::read("data/planets.txt").unwrap();
    println!("contents is {:?}", contents);
}
