use cgraph::{
    app::Window,
    object::primitives::{Color, Position, Size, create_quad},
};
use glam::Vec2;

fn main() {
    let mut win = Window::new("Dynamic Shadow Control", 800, 600, None);

    let mut object = create_quad(
        Size::new(200.0, 200.0),
        Color::new(0.6, 0.3, 0.9, 1.0),
        1.0,
        Position::new(400.0, 300.0),
    );

    object = object.with_shadow(20.0, Color::new(0.0, 0.0, 0.0, 0.5), Vec2::new(8.0, 8.0));

    win.add_object(object);

    win.launch();
}
