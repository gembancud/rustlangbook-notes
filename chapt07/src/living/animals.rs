use super::OrganicElements;
#[derive(Debug)]
pub struct Animal {
    pub name: String,
    element: OrganicElements,
}

impl Animal {
    pub fn new(name: &str) -> Animal {
        Animal {
            name: name.to_string(),
            element: OrganicElements {
                element: "Carbon".to_string(),
                atomic_number: 6,
            },
        }
    }
}
