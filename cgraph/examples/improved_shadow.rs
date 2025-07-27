use cgraph::{
    app::Window,
    object::primitives::{
        Color, Position, Size, create_circle_with_shadow, create_quad_with_shadow,
        create_rounded_quad_with_shadow,
    },
};
use glam::Vec2;

fn main() {
    let mut win = Window::new("Improved Shadows", 1200, 800);

    let object1 = create_quad_with_shadow(
        Size::new(200.0, 150.0),
        Color::new(0.8, 0.2, 0.2, 1.0),
        1.0,
        Position::new(200.0, 200.0),
        15.0,
        Color::new(0.0, 0.0, 0.0, 0.4),
        Vec2::new(5.0, 5.0),
    );

    let object2 = create_rounded_quad_with_shadow(
        Size::new(180.0, 180.0),
        Color::new(0.2, 0.8, 0.2, 1.0), // Green
        2.0,
        Position::new(500.0, 200.0),
        25.0,                           // corner radius
        20.0,                           // shadow radius
        Color::new(0.0, 0.0, 0.0, 0.3), // Black shadow with 30% opacity
        Vec2::new(8.0, 8.0),            // shadow offset
    );

    let object3 = create_circle_with_shadow(
        Size::new(160.0, 160.0),
        Color::new(0.2, 0.2, 0.8, 1.0), // Blue
        3.0,
        Position::new(800.0, 200.0),
        25.0,                           // shadow radius
        Color::new(0.1, 0.1, 0.3, 0.5), // Dark blue shadow
        Vec2::new(10.0, 10.0),          // shadow offset
    );

    let object4 = cgraph::object::primitives::create_quad(
        Size::new(150.0, 100.0),
        Color::new(0.8, 0.8, 0.2, 1.0), // Yellow
        4.0,
        Position::new(200.0, 500.0),
    )
    .with_shadow(
        12.0,                           // shadow radius
        Color::new(0.0, 0.0, 0.0, 0.6), // Black shadow with 60% opacity
        Vec2::new(3.0, 3.0),            // shadow offset
    );

    let mut object5 = cgraph::object::primitives::create_rounded_quad(
        Size::new(140.0, 140.0),
        Color::new(0.8, 0.4, 0.8, 1.0), // Purple
        5.0,
        Position::new(500.0, 500.0),
        20.0, // corner radius
    );
    object5.set_shadow(
        18.0,                           // shadow radius
        Color::new(0.3, 0.0, 0.3, 0.4), // Purple shadow
        Vec2::new(6.0, 6.0),            // shadow offset
    );

    let background = create_quad_with_shadow(
        Size::new(300.0, 200.0),
        Color::new(0.9, 0.9, 0.9, 1.0), // Light gray
        0.0,                            // Background z-index
        Position::new(800.0, 500.0),
        30.0,                           // Large shadow radius for soft effect
        Color::new(0.0, 0.0, 0.0, 0.2), // Subtle black shadow
        Vec2::new(15.0, 15.0),          // Large shadow offset
    );

    win.add_object(background);
    win.add_object(object1);
    win.add_object(object2);
    win.add_object(object3);
    win.add_object(object4);
    win.add_object(object5);

    win.launch();
}
