use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

use crate::renderer::create_renderer;

pub struct Context {
    pub name: String,
    pub version: String,
    pub description: String,
}

pub struct Window {
    pub title: String,
    pub width: u32,
    pub height: u32,
    pub update: Box<dyn Fn() -> ()>,
    renderer: Box<dyn crate::renderer::Renderer>,
    window: winit::window::Window,
    event_loop: EventLoop<()>,
}

impl Window {
    pub fn new(title: &str, width: u32, height: u32) -> Self {
        let event_loop = EventLoop::new();
        let window = WindowBuilder::new()
            .with_title(title)
            .with_inner_size(winit::dpi::LogicalSize::new(width, height))
            .build(&event_loop)
            .expect("Cannot create window");
        Window {
            title: title.to_string(),
            width,
            height,
            renderer: create_renderer(&window),
            window,
            event_loop,
        }
    }

    pub fn launch(self) -> () {
        let renderer = self.renderer;
        let window = self.window;

        renderer.resize(self.width as f64, self.height as f64);

        self.event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Wait;

            match event {
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::CloseRequested => {
                        renderer.destroy();
                        *control_flow = ControlFlow::Exit;
                    }
                    _ => (),
                },
                Event::MainEventsCleared => {
                    window.request_redraw();
                }
                Event::RedrawRequested(_) => {
                    renderer.render(&window);
                }
                _ => (),
            }
        });
    }

    pub fn add_object(&mut self, object: crate::object::Object) {
        self.renderer.add_object(object);
    }

    pub fn clear(&mut self) {
        self.renderer.clear();
    }

    pub fn destroy(&self) {
        self.renderer.destroy();
    }
}
