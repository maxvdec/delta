use outbox::component::Text;

fn main() {
    let mut app = outbox::app::Application::new("Outbox", "0.1.0");
    let mut win = outbox::window::Window::new("Main Window", 800, 600).without_decorations();

    let text = Text::new("Hello, World!");
    win.set_main_view(text);
    app.set_main_window(win);
    app.run();
}
