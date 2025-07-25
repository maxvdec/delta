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
}

impl Window {
    pub fn new(title: &str, width: u32, height: u32) -> Self {
        Window {
            title: title.to_string(),
            width,
            height,
        }
    }

    pub fn launch(&self) -> Option<()> {
        let event_loop = EventLoop::new();
        let window = WindowBuilder::new()
            .with_title(&self.title)
            .with_inner_size(winit::dpi::LogicalSize::new(self.width, self.height))
            .build(&event_loop)
            .ok()?;

        let renderer = create_renderer(&window);
        renderer.resize(self.width as f64, self.height as f64);

        event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Wait;

            match event {
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::CloseRequested => {
                        *control_flow = ControlFlow::Exit;
                    }
                    _ => (),
                },
                Event::MainEventsCleared => {
                    window.request_redraw();
                }
                Event::RedrawRequested(_) => {
                    renderer.render();
                }
                _ => (),
            }
        });
    }
}
