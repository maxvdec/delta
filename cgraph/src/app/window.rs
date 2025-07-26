use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

use crate::{app::SharedObjects, renderer::create_renderer};
#[cfg(target_os = "macos")]

pub struct Context {
    pub name: String,
    pub version: String,
    pub description: String,
}

pub struct Window {
    pub title: String,
    pub width: u32,
    pub height: u32,
    #[cfg(target_os = "macos")]
    pub update: Box<
        dyn Fn(
            &mut winit::window::Window,
            &mut crate::macos::metal::MetalRenderer,
            &mut SharedObjects,
        ) -> (),
    >,
    shared_objects: SharedObjects,
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
            update: Box::new(|_, _, _| ()),
            shared_objects: SharedObjects::new(),
        }
    }

    #[cfg(target_os = "macos")]
    pub fn each_frame<F>(&mut self, update: F)
    where
        F: Fn(
                &mut winit::window::Window,
                &mut crate::macos::metal::MetalRenderer,
                &mut SharedObjects,
            ) -> ()
            + 'static,
    {
        self.update = Box::new(update);
    }

    pub fn share_object<T: 'static + Send + Sync>(&mut self, object: T) {
        self.shared_objects.add_object(object);
    }

    pub fn launch(mut self) -> () {
        let mut window = self.window;
        let mut objects = self.shared_objects;
        let update = self.update;

        self.renderer.resize(self.width as f64, self.height as f64);

        self.event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Wait;

            match event {
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::CloseRequested => {
                        self.renderer.destroy();
                        *control_flow = ControlFlow::Exit;
                    }
                    _ => (),
                },
                Event::MainEventsCleared => {
                    window.request_redraw();
                }
                Event::RedrawRequested(_) => {
                    #[cfg(target_os = "macos")]
                    {
                        if let Some(metal_renderer) =
                            self.renderer
                                .as_any_mut()
                                .downcast_mut::<crate::macos::metal::MetalRenderer>()
                        {
                            (update)(&mut window, metal_renderer, &mut objects);
                        } else {
                            panic!("Renderer is not a MetalRenderer");
                        }
                    }
                    self.renderer.render(&window);
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
