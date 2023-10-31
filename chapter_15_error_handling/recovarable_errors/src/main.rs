use std::fs;

// included in the prelude, no need to explicitely include it into your programs
// enum Result<T,E> {
//     Ok(T),
//     Err(E)
// }

fn main() {
    let contents = fs::read_to_string("the_ultimate_question.txt");
    // contents is: Err(Os { code: 2, kind: NotFound, message: "No such file or directory" })
    println!("contents is: {:?}", contents);
    
    let contents = fs::read_to_string("42.txt");
    // contents is: Ok("42\n")
    println!("contents is: {:?}", contents);
    
    // if this reuslts in an Err, trying unwrap() will make the program panic!
    let contents = fs::read_to_string("42.txt").unwrap();
    //contents is: "42\n"
    println!("contents is: {:?}", contents);
    
    // custom Err message
    let contents = fs::read_to_string("the_ultimate_question.txt").expect("Nobody knows the ultimate question!");
    println!("contents is: {:?}", contents);
}
