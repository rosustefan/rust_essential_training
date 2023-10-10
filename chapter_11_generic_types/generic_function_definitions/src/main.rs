// Need to define the generic T with the PartialOrd trait to compare numerics
fn get_biggest<T: PartialOrd>(a: T, b: T) -> T {
    if a > b {
        a
    } else {
        b
    }
}

fn main() {
    println!("biggest is {}", get_biggest(1, 2));
}
