use std::fmt;

// setting bounds on datatypes T and U (to only accept compatible datatypes)
// fn compare_and_print<T: std::fmt::Display + PartialEq + From<U>, U: std::fmt::Display + PartialEq + Copy>(a: T, b: U) {
fn compare_and_print<T, U>(a: T, b: U) 
    where T: std::fmt::Display + PartialEq + From<U>,
          U: std::fmt::Display + PartialEq + Copy
    {
    if a == T::from(b) {
        println!("{} is equal to {}", a, b);
    } else {
        println!("{} is NOT equal to {}", a, b);
    }
}

fn main() {
    compare_and_print(1.0, 1);
    compare_and_print(1.1, 1);
    // next line will fail to compile; a `string-slice` cannot be converted into `float` value using the `from` method
    // compare_and_print(1.1, "one");
}