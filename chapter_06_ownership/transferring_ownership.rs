fn main() {
    let rocket_fuel_1 = 1;
    let rocket_fuel_2 = String::from("RP-1");

    // call the fn with the first variable and a copy of the 2nd variable
    process_fuel(rocket_fuel_1, rocket_fuel_2.clone());
    println!("rocket fuel is `{rocket_fuel_1}` and `{rocket_fuel_2}`");

    let rocket_fuel_3 = String::from("RP-3");
    let rocket_fuel_3 = process_fuel_returned(rocket_fuel_3);
    println!("rocket fuel is `{rocket_fuel_3}`");
}

// a copy of the `rocket_fuel_1` is passed to the fn implicitely (stack copy)
// a copy of the `rocket_fuel_2` needs to be passed to the fn explicitely (heap copy)
fn process_fuel(mut propellant_1: i32, mut propellant_2: String) {
    // propellant_1 copy of rocket_fuel is modified, not the original rocket_fuel
    propellant_1 += 1;
    propellant_2.push_str("/42");

    println!("[inside fn] processing propellant `{propellant_1}` and `{propellant_2}`...");
}

fn process_fuel_returned(propellant: String) -> String {
    println!("[inside fn] processing propellant `{propellant}`...");
    let new_fuel = String::from("LNG");
    new_fuel
}
