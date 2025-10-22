mod core;

use core::domain::{Pet, PetStatus, PetTag};

fn main() {
    let pet = Pet {
        id: 100,
        name: "Tommy".to_owned(),
        status: PetStatus::Available,
        category: "Dog".to_owned(),
        tags: vec![PetTag("Kind".to_owned(), "Puppy".to_owned())],
    };
    println!("Hello, here is my pet: {:?}", pet);
}
