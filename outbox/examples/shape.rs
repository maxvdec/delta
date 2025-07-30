use cgraph::object::primitives::Color;
use outbox::{app::Application, component::Shape, window::Window};

fn main() {
    let mut app = Application::new("Shapes", "0.1.0");
    let mut win = Window::new("Shapes", 800, 600);

    let quad = Shape::new_rectangle(100.0, 100.0, Color::new(1.0, 0.0, 0.0, 1.0));

    win.set_main_view(quad);

    app.set_main_window(win);
    app.run();
}
