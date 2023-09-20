use std::fs;
use std::io::prelude::*; // <- that is more commonly used than this: use std::io::Write;

fn main() {
    let mut speech = String::new();

    speech.push_str("We choose to go to the Moon in this decade\n");
    speech.push_str("and do the other things,\n");
    speech.push_str("not because they are easy,\n");
    speech.push_str("but because they are hard.\n");

    let _ = fs::write("data/speech.txt", speech);

    // append to an already existing file
    let mut file = fs::OpenOptions::new().append(true).open("data/planets.txt").unwrap();
    let _ = file.write(b"\nPluto");
}
