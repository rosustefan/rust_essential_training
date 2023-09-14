fn main() {
    // passing a string literal to the `message` variable
    // `::` is a path operator that allows access to the `from()` fn
    // the hardcoded text in the "" is `Earth`, a String literal => stored on the heap
    // `message` is stored on the stack and it points to the data stored on the heap
    let mut message = String::from("Earth"); // can be mutable
    println!("Original String is: {message}");
    message.push_str(" is home.");
    println!("Appended string is: {message}");
}
