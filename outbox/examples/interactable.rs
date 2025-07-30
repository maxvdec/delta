use cgraph::object::primitives::Color;
use outbox::{
    component::{Column, Shape, Text},
    stack,
    window::Window,
};

fn main() {
    let mut window = Window::new("Interactive Text Example", 800, 600);

    let view = stack!(
        Text::new("Click me!").on_click(|| {
            println!("Text clicked!");
        }),
        Shape::new_rectangle(200.0, 200.0, Color::new(1.0, 0.0, 1.0, 1.0))
    );
    window.set_main_view(view);
    window.launch();
}
