// PartialEq - equal only if all of the fields are equal
// PartialOrd - compares each field's chars in order of the struct's definition
#[derive(PartialEq, PartialOrd)]
struct Satellite {
    name: String,
    velocity: f64 // miles per second
}

fn main() {
    let hubble = Satellite {
        name: String::from("Hubble Telescope"),
        velocity: 4.72
    };
    
    let gps = Satellite {
        name: String::from("GPS"),
        velocity: 2.42
    };
    
    println!("hubble is == gps is {}", hubble == gps);
    println!("hubble is == gps is {}", hubble > gps);
}