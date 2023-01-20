pub struct Plant {
    pub name: String,
}

impl Plant {
    pub fn new(name: &str) -> Plant {
        Plant {
            name: name.to_string(),
        }
    }
}
