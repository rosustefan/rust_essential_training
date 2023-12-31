#[derive(Debug)]
enum Shape {
    Circle(f64), // radius
    Rectangle(f64, f64), // width, height
    Triangle(f64, f64, f64) // sides a, b, and c
}

fn main() {
    let my_shape = Shape::Rectangle(1.2, 3.4);
    println!("[INFO] my_shape is {:?}", my_shape);
    
    match my_shape {
        Shape::Circle(r) => println!("[Match] Circle with radius {}", r),
        Shape::Rectangle(w, h) => println!("[Match] {} x {} Rectangle", w, h),
        Shape::Triangle(a, b, c) => println!("[Match] Triangle with sides {}, {}, and {}", a, b, c)
    }
}
