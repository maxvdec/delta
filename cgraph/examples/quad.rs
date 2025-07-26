use cgraph::{
    app::Window,
    object::quad::{Color, Position, Size, create_quad, create_rounded_quad},
};

fn main() {
    let mut win = Window::new("Quad", 800, 600);

    create_quad(
        &mut win,
        Size::new(1000.0, 1000.0),
        Color::new(0.0, 1.0, 0.0, 1.0),
        0.0,
        Position::new(400.0, 300.0),
    );

    create_rounded_quad(
        &mut win,
        Size::new(200.0, 200.0),
        Color::new(1.0, 1.0, 0.0, 1.0),
        1.0,
        Position::new(401.0, 300.0),
        20.0,
    );

    win.launch();
}
