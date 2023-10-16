use std::fmt;

// the code calling the displayable fn doesn't need to know the exact return datatype
// ... just that is something, a datatype, that implements the Display trait

// fn get_displayable() -> impl fmt::Display {
//     13
// }

// fn get_displayable() -> impl fmt::Display {
//     "thirteen"
// }

// the Rust compiler needs to know what datatypes will display -> this fails to compile!
// this gets into the topic of `dynamic dispatch`, beyond scope of this course
fn get_displayable(choice: bool) -> impl fmt::Display {
    if choice {
        13
    } else {
        "thirteen"
    }
}

fn main() {
    println!("output is {}", get_displayable(true));
}
