use cgraph::{
    self,
    app::{CoreEvent, CoreEventReference, CoreWindowEvent, Window, WindowOptions},
};

fn main() {
    let mut win = Window::new("Window", 800, 600, Some(WindowOptions::no_titlebar()));
    win.on_event(CoreEventReference::WindowEvent, |_, event| {
        if let CoreEvent::WindowEvent(CoreWindowEvent::KeyboardInput(input)) = event {
            println!("Key pressed: {input:?}");
        }
    });
    win.launch();
}
