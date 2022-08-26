pub struct Animal {
    pub name: String,
}

impl Animal {
    pub fn new(name: &str) -> Animal {
        Animal {
            name: name.to_string(),
        }
    }
}
