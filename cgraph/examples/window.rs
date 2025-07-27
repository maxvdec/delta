use cgraph::{
    self,
    app::{CoreEvent, CoreEventReference, CoreWindowEvent, Window},
};

fn main() {
    let mut win = Window::new("Window", 800, 600);
    win.on_event(CoreEventReference::WindowEvent, |_, event| match event {
        CoreEvent::WindowEvent(event) => match event {
            CoreWindowEvent::KeyboardInput(input) => {
                println!("Key pressed: {:?}", input);
            }
            _ => (),
        },
        _ => (),
    });
    win.launch();
}
