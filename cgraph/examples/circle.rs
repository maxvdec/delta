use cgraph::{
    app::Window,
    object::primitives::{Color, Position, Size, create_circle},
};

fn main() {
    let mut win = Window::new("Circle", 800, 600, None);

    win.add_object(create_circle(
        Size::new(1000.0, 1000.0),
        Color::new(0.0, 1.0, 0.0, 1.0),
        0.0,
        Position::new(400.0, 300.0),
    ));

    win.add_object(create_circle(
        Size::new(200.0, 200.0),
        Color::new(1.0, 1.0, 0.0, 1.0),
        1.0,
        Position::new(0.0, 0.0),
    ));

    win.launch();
}
