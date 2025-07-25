pub struct Vertex {
    pub position: [f32; 2],
    pub color: [f32; 4],
}

pub struct Object {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
}

pub trait Renderer {
    fn new(window: &winit::window::Window) -> Self
    where
        Self: Sized;
    fn render(&self);
    fn resize(&self, width: f64, height: f64);
    fn destroy(&self);
    fn add_object(&mut self, object: Object);
    fn clear(&mut self);
}

pub fn create_renderer(window: &winit::window::Window) -> Box<dyn Renderer> {
    if cfg!(target_os = "macos") {
        use crate::macos::metal::MetalRenderer;
        Box::new(MetalRenderer::new(window))
    } else {
        panic!("Unsupported platform");
    }
}
