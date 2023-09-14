fn main() {
    let mut count = 0;

    let result = loop {
        if count == 13 {
            break count * 10; // break and return value
        }

        count += 1;
        println!("Count is {}", count);
    };

    println!("The result is {}", result);
}
