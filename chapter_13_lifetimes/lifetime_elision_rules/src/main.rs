// lifetime elision rules = set of rules for the compiler to analyze reference lifetimes
// describes situations that do not require explicit lifetime annotations
// if any ambiguity remains, explicit annotation will be required
//
// there are currently three (compiler) lifetime elision rules:
// #1 each input parameter is a reference is assigned its own lifetime
// #2 if there is exactly one input lifetime, assign it to all output lifetimes
// #3 if there is a `&self` or `&mut self` input parameter, its lifetime will be assigned to all output lifetimes

fn main() {
    let message = String::from("Greetings from Earth!");
    let first_word = get_first_word(&message);
    
    println!("first_word is {}", first_word);
}

fn get_first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    
    for (index, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..index]; // found a space
        }
    }
    
    &s // no spaces found; input a single word
}