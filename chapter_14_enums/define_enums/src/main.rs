#[derive(Debug)]
enum Shape {
    Circle(f64),
    Rectangle(f64, f64),
    Triangle(f64, f64, f64)
}

// Example of other use case for a drawing program
// enum Command {
//     Clear,
//     DrawLine(f64, f64), // x, y
//     DrawShape(Shape)
// }

fn main() {
    let my_shape = Shape::Rectangle(1.2, 3.4);
    println!("my_shape is {:?}", my_shape);
}
