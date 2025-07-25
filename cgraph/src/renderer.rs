use crate::object::buffer::Buffer;
use crate::object::{Object, Vertex};

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

impl Object {
    pub fn new(vertices: Vec<Vertex>, indices: Vec<u32>) -> Self {
        let buffer = Buffer::new(vertices.clone());
        let index_buffer = Buffer::new(indices.clone());
        Object {
            vertices,
            indices,
            buffer,
            index_buffer,
        }
    }

    pub fn add_vertex(&mut self, vertex: Vertex) {
        self.vertices.push(vertex);
    }

    pub fn add_index(&mut self, index: u32) {
        self.indices.push(index);
    }
}
