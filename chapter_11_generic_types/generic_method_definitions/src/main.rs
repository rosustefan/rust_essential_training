// Generics are a zero-code abstraction => no slow down of the program due to monomorphization
#[derive(Debug)]
struct Rectangle<T, U> {
    width: T,
    height: U,
}

// method that works with Rectangles with generics
impl<T, U> Rectangle<T, U> {
    fn get_width(&self) -> &T {
        &self.width
    }
}

// method that works only with Rectangles with u8 data types
impl Rectangle<u8, u8> {
    fn get_perimeter(&self) -> u8 {
        2 * self.width + 2 * self.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 1u16,
        height: 3f32,
    };

    let rect2 = Rectangle {
        width: 1u8,
        height: 3u8
    };   

    println!("rect is {:?}", rect1);
    println!("width is {}", rect1.get_width());
    println!("perimeter is {}", rect2.get_perimeter());
}
