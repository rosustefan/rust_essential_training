#[derive(Debug)]
#[derive(Clone)]
struct Shuttle {
    name: String,
    crew_size: u8,
    propellant: f64,
}

fn main() {
    let mut vehicle = Shuttle {
        name: String::from("Endeavour"),
        crew_size: 7,
        propellant: 835948.0,
    };

    let vehicle2 = Shuttle {
        name: String::from("Discovery"),
        ..vehicle
    };

    let vehicle3 = Shuttle {
        ..vehicle.clone()
    };

vehicle.name = String::from("Atlantis");
println!("vehicle1 is {:?}", vehicle);
println!("vehicle2 is {:?}", vehicle2);
println!("vehicle3 is {:?}", vehicle3);
}
