fn main() {
    let result = square_expression(13);
    println!("result is {}", result);

    let result = square_statement(13);
    println!("result is {:?}", result);
}

fn square_expression(x: i32) -> i32 {
    println!("squaring {}", x);
    // last line in a Rust fn will be the return output
    // w/o a semicolon this evaluates to an expression, not a statement
    x * x
}

fn square_statement(x: i32) -> (i32, i32) {
    println!("squaring {}", x);
    // w a semicolon this returns a statement, not an expression
    // return keyword ignores all code that follows
    return (x, x * x); // return a tuple
}