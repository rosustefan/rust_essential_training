use std::fs;
use std::io;

fn read_and_combine(f1: &str, f2: &str) -> Result<String, io::Error> {
    // the verbose match method
    // let mut s1 = match fs::read_to_string(f1) {
    //     Ok(s) => s,
    //     Err(e) => return Err(e)
    // };
    // ... and using the Rust shorthand equivalent code
    let mut s1 = fs::read_to_string(f1)?;
    
    let s2 = match fs::read_to_string(f2) {
        Ok(s) => s,
        Err(e) => return Err(e)
    };
    
    s1.push('\n');
    s1.push_str(&s2);
    Ok(s1)
}

fn main() {
    let result = read_and_combine("planets.txt", "dwarf_planets.txt");
    match result {
        Ok(s) => println!("result is...\n{}", s),
        Err(e) => println!("There was an error: {}", e)
    };
    
    // forcing an error by using a file that does not exist
    
    let result = read_and_combine("planet.txt", "dwarf_planets.txt");
    match result {
        Ok(s) => println!("result is...\n{}", s),
        Err(e) => println!("There was an error: {}", e)
    };
}
