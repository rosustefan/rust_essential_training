fn main() {
    let planet = "Earth";
    println!("[global scope] planet is {planet}");

    let planet = "Mars"; // declaring a new variable with the `let` kw
    println!("[global scope after re-declaring] planet is {planet}");
    {
        let planet = 4; // shadowing variable inside this block of code
        println!("[shadowing] planet is {planet}");
    }
    println!("[global scope] planet is {planet}") // return to global scope value
}
