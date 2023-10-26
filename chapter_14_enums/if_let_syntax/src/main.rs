fn main() {
    let number = Some(13);
    
    match number {
        Some(13) => println!("thirteen"),
        _ => () // if anything else `_` do nothing `()`
    }
    
    // the equivalent Rust-way to do this
    if let Some(13) = number {
        println!("thirteen")
    }
}