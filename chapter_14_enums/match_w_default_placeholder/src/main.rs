fn main() {
    let my_number = 4u8; // 256 possible values
    
    let result = match my_number {
        0 => "zero",
        1 => "one",
        2 => "two",
        3 => "three",
        // wildcard pattern `_` == default case that will be executed 
        // if none of the other match-arms match
        _ => {
            println!("{} did not match", my_number);
            "something else"
        }
    };
    println!("result is {}", result);
}
