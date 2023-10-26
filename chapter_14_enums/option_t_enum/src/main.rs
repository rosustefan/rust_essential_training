// the `Null` value concept implemented in Rust as an `enum`
// enum Option<T> {
//     Some(T),
//     None
// }

fn main() {
    let countdown = [5, 4, 3, 2, 1];
    
    // index out of bounds: the length is 5 but the index is 5
    // let number = countdown[5];
    
    // this will work and return `None`
    let number = countdown.get(5);
    println!("number is {:?}", number);
    
    let number = countdown.get(4);
    println!("number is {:?}", number);
    
    // error[E0369]: cannot add `{integer}` to `Option<&{integer}>`
    // let number = countdown.get(4);
    // let number = number + 1;
    // println!("number is {:?}", number);
    
    let number = countdown.get(5);
    // unwrap value or return a refference to 0
    let number = number.unwrap_or(&0) + 1;
    println!("number is {:?}", number);
}