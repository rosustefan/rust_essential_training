use std::io;

fn main() {
    let mut buffer = String::new();

    println!("Enter a message:");
    let _ = io::stdin().read_line(&mut buffer);
    print!("buffer is {buffer}");

    // 1. `trim()` elminate white spaces (`\n` is part of the input above)
    // 2. `parse()` the String into an i32 integer (actually it's a `result enum`)
    // 3. `unwrap()` the `result enum`
    let number = buffer.trim().parse::<i32>().unwrap(); // == let number: i32 = buffer.trim().parse().unwrap();
    println!("number + 1 is {}", number + 1);
}
