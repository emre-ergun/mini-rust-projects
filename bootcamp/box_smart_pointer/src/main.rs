trait UIComponent {
    fn render(&self) {
        println!("Rendereing component...");
    }
}

struct Button {
    text: String,
}

impl UIComponent for Button {}

struct Container {
    name: String,
    child: Box<Container>,
}

impl UIComponent for Container {}

fn main() {
    let button_a = Button {
        text: "Button a".to_owned(),
    };
    let button_b = Box::new(Button {
        text: "Button b".to_owned(),
    });

    // ownership of the button a is transferred to button c
    // entire button a will be copied, because button is on the stack
    let button_c = button_a;
    // ownership of the button b is transferred to button d
    // only the box smart pointer will be copied, which is a small amount of data.
    // actual button will be stored on the heap
    let button_d = button_b;

    let components: Vec<Box<dyn UIComponent>> = vec![Box::new(button_c), button_d];
}
