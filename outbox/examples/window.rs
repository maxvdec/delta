use outbox::{app::Application, window::Window};

fn main() {
    let mut app = Application::new("Window", "0.1.0");
    let win = Window::new("Window", 800, 600);
    app.set_main_window(win);
    app.run();
}
