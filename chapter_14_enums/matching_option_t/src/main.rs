fn main() {
    let countdown = [5, 4, 3, 2, 1];
    let number = countdown.get(5);
    // return the value if there is `Some(number)`, else return 0
    let number = match number {
        Some(number) => number + 1,
        None => 0
    };
    println!("number is {:?}", number);
    
    //
    
    let number = countdown.get(3);
    let number = match number {
        Some(number) => number + 1,
        None => 0
    };
    println!("number is {:?}", number);
}
