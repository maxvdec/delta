use outbox::{
    app::Application,
    component::{Column, Text},
    renderable::Renderable,
    stack,
    window::Window,
};

fn main() {
    let mut app = Application::new("Window", "0.1.0");
    let mut win = Window::new("Window", 800, 600);

    let mut col: Column = stack!(
        Text::new("Hello, World!"),
        Text::new("This is a column layout!"),
        Text::new("Ain't this amazing?"),
        Text::new("Hello"),
        Text::new("This is Outbox!")
    );

    col.add_spacing(10.0);
    col.padding([10.0, 100.0]);

    win.set_main_view(col);

    app.set_main_window(win);
    app.run();
}
