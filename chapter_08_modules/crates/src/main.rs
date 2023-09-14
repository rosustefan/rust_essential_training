// cannot use this here because it creates ambiguity with the random fn
// use rand::random;

// use rand;
use rand::prelude::*;

fn main() {
    let number1 = rand::random::<f64>();
    println!("number1 is {}", number1);

    let number2 = random();
    println!("number2 is {}", number2);

    let number3 = thread_rng().gen_range(1..42);
    println!("number3 is {}", number3);
}

fn random() -> f64 {
    1.0
}
