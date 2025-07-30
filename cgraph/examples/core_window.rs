use cgraph::{
    self,
    app::{CoreEvent, CoreEventReference, CoreWindowEvent, Window},
};

fn main() {
    let mut win = Window::new("Window", 800, 600, None);
    win.on_event(CoreEventReference::WindowEvent, |_, event, _| {
        if let CoreEvent::WindowEvent(CoreWindowEvent::KeyboardInput(input)) = event {
            println!("Key pressed: {input:?}");
        }
    });
    win.launch();
}
