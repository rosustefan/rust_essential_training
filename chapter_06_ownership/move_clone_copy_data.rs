// data types with a known size are copied implicitely => they are stored on the stack
// data types with an unknown size need to be copied explicitely using .clone() => they are stored on the heap
fn main() {
    let outer_planet: String;
    {
        let mut inner_planet = String::from("Mercury");
        println!("inner planet is {inner_planet}");

        outer_planet = inner_planet; // outer_planet becomes the new owner of the "Memory" String => move

        let mut other_planet = outer_planet.clone(); // other_planet duplicates the data "Memory" to another heap location
        println!("other planet is {other_planet}");
        other_planet.clear(); // clear the contents of other_planet
        println!("other planet is {other_planet}");
    }
    println!("outer planet is {outer_planet}")
}
