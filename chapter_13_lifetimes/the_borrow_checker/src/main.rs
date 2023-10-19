fn main() {
    let propellant;
    {
        let rp1 = String::from("RP-1");
        propellant = &rp1; // borrowed value does not live long enough
        println!("propellant is {}", propellant);
    }
    println!("propellant is {}", propellant); // borrow later used here
}