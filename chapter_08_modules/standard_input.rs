use std::io;

fn main() {
    let mut buffer = String::new();

    println!("Enter a message:");
    let _ = io::stdin().read_line(&mut buffer);

    println!("buffer is {}", buffer);
}
