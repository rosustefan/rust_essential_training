use std::mem;

struct Shuttle {
    name: String,
    crew_size: u8,
    propellant: f64
}

fn main() {
    let vehicle = Shuttle {
        name: String::from("Atlantis"),
        crew_size: 7,
        propellant: 835958.0
    };
    println!("vehicle size on stack: {} bytes", mem::size_of_val(&vehicle));
    
    // allocate memory on the heap to hold the data -> boxed_vehicle becomes the owner
    let boxed_vehicle: Box<Shuttle> = Box::new(vehicle);
    // size of pointer on stack
    println!("boxed_vehicle size on stack: {} bytes", mem::size_of_val(&boxed_vehicle)); 
    // size of refferenced data on the heap; * is the derefferencing operator just as in C
    println!("boxed_vehicle size on heap: {} bytes", mem::size_of_val(&*boxed_vehicle));

    // move data back from the heap to the stack == derefference it
    let unboxed_vehicle: Shuttle = *boxed_vehicle;
    println!("unboxed_vehicle size on stack: {} bytes", mem::size_of_val(&unboxed_vehicle));
}
