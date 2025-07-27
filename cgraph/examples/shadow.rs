use cgraph::{
    app::Window,
    object::primitives::{Color, Position, Size, create_quad, create_rounded_quad},
};
use glam::Vec2;

fn main() {
    let mut win = Window::new("Quad", 800, 600);

    let mut object = create_quad(
        Size::new(1000.0, 1000.0),
        Color::new(0.0, 1.0, 0.0, 1.0),
        0.0,
        Position::new(400.0, 300.0),
    );

    object.shadow_radius = 20.0;
    object.shadow_color = Color::new(0.0, 0.0, 0.0, 0.5);
    object.shadow_on = true;

    let mut object2 = create_rounded_quad(
        Size::new(200.0, 200.0),
        Color::new(1.0, 1.0, 0.0, 1.0),
        1.0,
        Position::new(400.0, 300.0),
        20.0,
    );
    object2.shadow_radius = 10.0;
    object2.shadow_color = Color::new(0.0, 0.0, 0.0, 0.5);
    object2.shadow_offset = Vec2::new(0.0, 20.0);
    object2.shadow_on = true;

    win.add_object(object);
    win.add_object(object2);

    win.launch();
}
