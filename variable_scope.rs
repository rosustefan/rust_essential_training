fn main() {
    if true {
        let planet = "Earth"; // `planet` in scope inside this `if` code block
        println!("planet is {}", planet);
    }
    // println!("planet is {}", planet);  // `not found in this scope` <= `planet` went out of scope here
}
