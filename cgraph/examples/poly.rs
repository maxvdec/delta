use cgraph::{
    app::Window,
    object::primitives::{Color, Position, Size, create_polygon},
};

fn main() {
    let mut win = Window::new("Quad", 800, 600);

    create_polygon(
        &mut win,
        Size::new(1000.0, 1000.0),
        Color::new(0.0, 1.0, 0.0, 1.0),
        0.0,
        Position::new(400.0, 300.0),
        6,
    );

    create_polygon(
        &mut win,
        Size::new(200.0, 200.0),
        Color::new(1.0, 1.0, 0.0, 1.0),
        1.0,
        Position::new(400.0, 300.0),
        8,
    );

    win.launch();
}
