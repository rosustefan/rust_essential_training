// #[derive(Debug)]
struct Cats {
    name: String,
    age: u8,
}

fn main() {
    let Coco = Cats {
        name: String::from("Coco"),
        age: 7,
    };

    let Rosie = Cats {
        name: String::from("Rosie"),
        age: 7,
    };
    
    println!("{} {}", Coco.name, Coco.age);
    println!("{} {}", Rosie.name, Rosie.age);
}
