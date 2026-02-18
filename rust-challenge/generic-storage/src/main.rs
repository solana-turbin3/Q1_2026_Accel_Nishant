use generic_storage::{Storage, Borsh, Person};

fn main() {
    let person = Person {
        name: "Andre".to_string(),
        age: 30,
    };

    let mut storage = Storage::new(Borsh);
    storage.save(&person).unwrap();

    let loaded = storage.load().unwrap();
    println!("Loaded: {:?}", loaded);
}