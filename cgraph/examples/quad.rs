use cgraph::{
    app::Window,
    object::primitives::{Color, Position, Size, create_quad, create_rounded_quad},
};

fn main() {
    let mut win = Window::new("Quad", 800, 600, None);

    win.add_object(create_quad(
        Size::new(1000.0, 1000.0),
        Color::new(0.0, 1.0, 0.0, 1.0),
        0.0,
        Position::new(400.0, 300.0),
    ));

    win.add_object(create_rounded_quad(
        Size::new(200.0, 200.0),
        Color::new(1.0, 1.0, 0.0, 1.0),
        1.0,
        Position::new(400.0, 300.0),
        20.0,
    ));

    win.launch();
}
