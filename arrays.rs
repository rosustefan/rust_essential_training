fn main() {
    let mut letters = ['a', 'b', 'c'];
    letters[0] = 'x';
    let first_letter = letters[0];
    println!("first_letter is {}", first_letter);

    let numbers: [i32; 5]; // i32 data-type and having 5 elements (this is a repeat expression)
    numbers = [0; 5]; // equivalent to initializing the array manually: numbers = [0, 0, 0, 0, 0];
    let index = numbers.len(); // length is 5, but last index is 4
    println!("last number is {}", numbers[index]);
}