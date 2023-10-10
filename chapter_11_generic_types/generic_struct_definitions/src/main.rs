// Generics are a zero-code abstraction => no slow down of the program due to monomorphization
#[derive(Debug)]
struct Rectangle<T, U> {
    width: T,
    height: U,
}

fn main() {
    let rect = Rectangle {
        width: 1u16,
        height: 3f32,
    };
    println!("rect is {:?}", rect);
}
