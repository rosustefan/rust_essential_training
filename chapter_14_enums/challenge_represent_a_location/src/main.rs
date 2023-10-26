/* YOUR CODE GOES HERE */
enum Location {
    Unknown,
    Anonymous,
    Known(f64, f64) // latitude and longitude
}

impl Location {
    fn display(&self) {
        match *self {
            Location::Unknown => println!("Unknown location!"),
            Location::Anonymous => println!("Anonymous location!"),
            Location::Known(lat, lon) => println!("Latitude: {} | Longitude: {}", lat, lon)
        }
    }
}

fn main() {
    let address = Location::Unknown;
    address.display();
    
    let address = Location::Anonymous;
    address.display();
    
    let address = Location::Known(28.608295, -80.604177);
    address.display();
}