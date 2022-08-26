pub use crate::living::animals::Animal;

pub fn create_animal(name: &str) -> Animal {
    Animal {
        name: name.to_string(),
    }
}
