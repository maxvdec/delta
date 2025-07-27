use glam::{Vec2, Vec4};

use crate::object::buffer::Buffer;
use crate::object::{Object, Vertex};

pub trait Renderer {
    fn new(window: &winit::window::Window) -> Self
    where
        Self: Sized;
    fn render(&mut self, window: &winit::window::Window);
    fn resize(&mut self, width: f64, height: f64);
    fn destroy(&self);
    fn add_object(&mut self, object: Object);
    fn clear(&mut self);
    fn as_any(&self) -> &dyn std::any::Any;
    #[allow(dead_code)]
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any;
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
            position: Vec2::new(0.0, 0.0),
            scale: Vec2::new(1.0, 1.0),
            original_pixel_size: Vec2::new(1.0, 1.0),
            rotation: 0.0,
            corner_radius: 0.0,
            #[cfg(target_os = "macos")]
            texture: None,
            use_texture: false,
            shadow_radius: 0.0,
            shadow_color: Vec4::new(0.0, 0.0, 0.0, 0.0),
            shadow_offset: Vec2::new(0.0, 0.0),
            shadow_on: false,
        }
    }

    pub fn add_vertex(&mut self, vertex: Vertex) {
        self.vertices.push(vertex);
    }

    pub fn add_index(&mut self, index: u32) {
        self.indices.push(index);
    }

    #[cfg(target_os = "macos")]
    pub fn with_texture(mut self, texture: crate::macos::image::Image) -> Self {
        self.texture = Some(texture);
        self.use_texture = true;
        self
    }

    pub fn set_use_texture(&mut self, use_texture: bool) {
        self.use_texture = use_texture;
    }
}
