use outbox::{
    app::Application,
    component::{Column, Row, Text},
    row, stack,
    window::Window,
};

fn main() {
    let mut app = Application::new("Window", "0.1.0");
    let mut win = Window::new("Window", 800, 600);

    let mut col: Column = stack!(
        Text::new_default("Hello, World!", &win)
            .italic()
            .extra_bold(),
        Text::new_default("This is a column layout!", &win),
        Text::new_default("Ain't this amazing?", &win),
        Text::new_default("Hello", &win),
        Text::new_default("This is Outbox!", &win),
        row!(
            Text::new("This is a row layout!"),
            Text::new("With multiple elements!"),
        )
        .add_spacing(10.0)
    );

    col.add_spacing(30.0);

    win.set_main_view(col);

    app.set_main_window(win);
    app.run();
}
