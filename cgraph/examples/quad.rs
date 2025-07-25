use cgraph::{
    app::Window,
    object::quad::{Position, Size, create_quad},
};

fn main() {
    let mut win = Window::new("Quad", 800, 600);

    create_quad(
        &mut win,
        Size::new(1.0, 1.0),
        [0.0, 1.0, 0.0, 1.0],
        0.0,
        Position::new(-0.5, -0.5),
    );

    win.launch();
}
