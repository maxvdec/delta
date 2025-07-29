use cgraph::app::Window;
use cgraph::object::primitives::Color;
use cgraph::text::*;
use glam::Vec2;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut window = Window::new("Hello", 800, 600, None);

    let style = TextStyleBuilder::new("Arial", 24.0)
        .italic()
        .bold()
        .underlined()
        .build_font(&window)?;

    let text_object = make_styled_text(
        style,
        "Hello!",
        Color::new(0.0, 1.0, 0.0, 1.0),
        20.0,
        Vec2::new(100.0, 100.0),
    )?;

    window.add_object(text_object);

    window.launch();

    Ok(())
}
