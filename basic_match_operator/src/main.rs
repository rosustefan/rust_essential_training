fn main() {
    let command = "Hello, World!";
    
    let result = match command {
        "Hello, world!" => "Hello to you too!",
        "Goodbye, World!" => "See you later",
        _ => {
            println!("{} did not match", command);
            "No match found"
        }
    };
    println!("result is {}", result);
}