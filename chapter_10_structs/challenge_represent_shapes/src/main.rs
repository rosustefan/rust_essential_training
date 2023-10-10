struct Rectangle {
    width: f32,
    height: f32,
}

impl Rectangle {
    fn new(width: f32, height: f32) -> Rectangle {
       Rectangle {
           width,
           height,
       }
    }

    fn get_area(&self) -> f32 {
        // Format the area with exactly two decimal places
        let formatted_area = format!("{:.2}", self.width * self.height);
        // Parse the formatted string back into an f32
        let parsed_area = formatted_area.parse::<f32>().unwrap();
        return parsed_area
    }

    fn scale(&mut self, scale: f32) {
        self.width = self.width * scale;
        self.height = self.height * scale
    }
}

fn main() {
    let mut rect = Rectangle::new(1.2, 3.4);
    assert_eq!(rect.get_area(), 4.08);
    rect.scale(0.5);
    assert_eq!(rect.get_area(), 1.02);
    println!("Tests passed!");
}
