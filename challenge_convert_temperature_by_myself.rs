fn main() {
    let temp_in_celsius = 29.0;
    let temp_in_fahrenheit = celsius_to_fahrenheit(temp_in_celsius);
    
    println!("{} degrees Celsius equals to {} degrees Fahrenheit.", temp_in_celsius, temp_in_fahrenheit);
}

fn celsius_to_fahrenheit(temp: f32) -> f32 {
    1.8 * temp + 32.0
}