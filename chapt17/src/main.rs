mod gui;

use gui::{Button, Draw, Screen, TextField};

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        // Code to actually draw a select box
        println!(
            "Drawing SelectBox with width: {}, height: {}, and options: {:?}",
            self.width, self.height, self.options
        );
    }
}

fn main() {
    let _screen = Screen {
        components: vec![
            Box::new(Button {
                width: 75,
                height: 10,
                label: String::from("Button"),
            }),
            Box::new(SelectBox {
                width: 50,
                height: 10,
                options: vec![String::from("Maybe"), String::from("No")],
            }),
            Box::new(TextField {
                text: String::from("Hello TextField"),
            }),
        ],
    };
    _screen.run();
}
