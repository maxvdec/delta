use cgraph::{
    app::Window,
    object::primitives::{Color, Position, Size, create_quad, create_rounded_quad},
};

fn main() {
    let mut win = Window::new("Quad", 800, 600, None);

    win.add_object(create_quad(
        Size::new(800.0, 600.0),
        Color::new(0.0, 1.0, 0.0, 1.0),
        0.0,
        Position::new(0.0, 0.0),
    ));

    win.add_object(create_rounded_quad(
        Size::new(100.0, 100.0),
        Color::new(1.0, 1.0, 0.0, 1.0),
        1.0,
        Position::new(350.0, 250.0),
        20.0,
    ));

    win.launch();
}
