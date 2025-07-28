use cgraph::{
    app::Window,
    image::Image,
    object::primitives::{Color, Position, Size, create_rounded_quad},
};

fn main() {
    let mut win = Window::new("Texture", 800, 600, None);

    let mut object = create_rounded_quad(
        Size::new(1000.0, 1000.0),
        Color::new(0.0, 1.0, 0.0, 1.0),
        0.0,
        Position::new(400.0, 300.0),
        50.0,
    );

    object = object.with_texture(
        Image::new(&format!("{}/assets/wall.jpg", env!("CARGO_MANIFEST_DIR"))).unwrap(),
    );

    win.add_object(object);

    win.launch();
}
