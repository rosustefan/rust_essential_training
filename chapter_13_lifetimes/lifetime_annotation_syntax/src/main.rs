// fn best_fuel(x: &str, y: &str) -> &str { // without lifetime annotations
fn best_fuel<'a>(x: &'a str, y: &'a str) -> &'a str { // with lifetime annotations
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let result;
    let propellant1 = String::from("RP-1");
    let propellant2 = String::from("LNG");
    
    result = best_fuel(&propellant1, &propellant2);
    
    println!("result is {}", result);
}