fn main() {
    let mut stuff: (u8, f32, char) = (10, 3.14, 'x');
    stuff.0 += 3;
    let (first_item, second_item) = (stuff.0, stuff.1);
    println!("first_item is {}\nsecond item is {}", first_item, second_item);

    // destructuring
    let (a, b, c) = stuff;
    println!("b is {}", b);
}