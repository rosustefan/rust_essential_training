fn main() {
    say_hello();

    let x = 1;
    let y = 2;
    say_the_sum(x, y);
    say_a_number(x as i32);
}

fn say_hello() {
    println!("Hello, World!");
    say_a_number(13); // argument `13`
}

fn say_a_number(number: i32) { // parameter `number: i32`
    println!("number is {}", number);
}

fn say_the_sum(a: u8, b: u8) { // the compiler recognizes that we use `u8` inputs, therefore `x` and `y` will be `u8` at declaration
    let sum = a + b;
    println!("a + b = {}", sum);
}