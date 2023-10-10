#[derive(Debug)]
struct Rectangle {
    width: f64,
    height: f64,
}

fn main() {
    let rect = Rectangle {
        width: 1,
        height: 3,
    };
    println!("rect is {:?}", rect);
}
