use cgraph::{
    self,
    app::Window,
    object::primitives::Color,
    text::{get_font, make_text},
};
use glam::Vec2;

fn main() {
    let mut win = Window::new("Window", 800, 600, None);

    let font = get_font(&win, "BIZ UDMincho", 18.0).unwrap();
    let object = make_text(
        font,
        "おはよう!",
        Color::new(0.0, 1.0, 0.0, 1.0),
        3.0,
        Vec2::new(100.0, 100.0),
    )
    .unwrap();

    win.add_object(object);

    win.launch();
}
