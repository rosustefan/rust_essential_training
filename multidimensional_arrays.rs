fn main() {
    let parking_lot = [
        [1, 2, 3],
        [4, 5, 6],
    ];
    let number = parking_lot[0][1];
    println!("the value of the retrieved number is {}", number);

    let garage: [[[i32; 100]; 20]; 5]; // 3-dimensional array with 5*20*100 i32 elements
    let garage_zero = [[[0; 100]; 20]; 5]; // 3-dimensional array with 5*20*100 `0` elements
}