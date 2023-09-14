fn main() {
    let make_x_odd = true;

    let (x, y) = if make_x_odd { (1, "odd") } else { (2, "even") };

    println!("x is {} and it's an {} number.", x, y);
}
