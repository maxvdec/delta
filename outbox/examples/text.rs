use outbox::component::Text;

fn main() {
    let mut app = outbox::app::Application::new("Outbox", "0.1.0");
    let mut win = outbox::window::Window::new("Main Window", 800, 600);

    let mut text = Text::new("Hello, World!");
    text.set_font_by_name("Arial").unwrap();
    text = text.set_size(24.0).italic().extra_bold();

    win.set_main_view(text);
    app.set_main_window(win);
    app.run();
}
