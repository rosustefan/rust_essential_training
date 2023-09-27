#[derive(Debug)]
struct Shuttle {
    name: String,
    crew_size: u8,
    propellant: f64
}

impl Shuttle {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn add_fuel(&mut self, gallons: f64) {
        self.propellant += gallons;
    }

    fn new(name: &str) -> Shuttle {
        Shuttle {
            name: String::from(name),
            crew_size: 7,
            propellant: 0.0
        }
    }
}




fn main() {
    let mut vehicle1 = Shuttle::new("Endeavour");
    let vehicle2 = Shuttle::new("Discovery");
    println!("vehicle2 is {:?}", vehicle2);

    let vehicle1_name = vehicle1.get_name();
    println!("vehicle_name is {}", vehicle1_name);

    println!("propellant is {}", vehicle1.propellant);
    vehicle1.add_fuel(42.0);
    println!("propellant is {}", vehicle1.propellant);
}
