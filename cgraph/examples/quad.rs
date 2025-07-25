use cgraph::{
    app::Window,
    object::quad::{Color, Position, Size, create_quad},
};

fn main() {
    let mut win = Window::new("Quad", 800, 600);

    create_quad(
        &mut win,
        Size::new(400.0, 400.0),
        Color::new(0.0, 1.0, 0.0, 1.0),
        40.0,
        Position::new(0.0, 0.0),
    );

    create_quad(
        &mut win,
        Size::new(200.0, 200.0),
        Color::new(1.0, 1.0, 0.0, 1.0),
        50.0,
        Position::new(0.0, 0.0),
    );

    win.launch();
}
