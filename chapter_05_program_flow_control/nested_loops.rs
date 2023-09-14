fn main() {
    // let matrix = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];

    // for row in matrix.iter() {
    //     for num in row.iter() {
    //         print!("{}  ", num);
    //     }
    //     println!("\n");
    // }

    let mut matrix = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];

    for row in matrix.iter_mut() {
        for num in row.iter_mut() {
            *num += 42;
            print!("{}  ", num);
        }
        println!("\n");
    }
}
