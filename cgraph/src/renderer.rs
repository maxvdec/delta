use glam::{Vec2, Vec4};

use crate::object::buffer::Buffer;
use crate::object::primitives::Color;
use crate::object::{Object, Vertex};

pub trait Renderer {
    fn new(window: &winit::window::Window, background_color: Color) -> Self
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
    fn set_background_color(&mut self, background_color: Color);
}

pub fn create_renderer(window: &winit::window::Window, color: Color) -> Box<dyn Renderer> {
    if cfg!(target_os = "macos") {
        use crate::macos::metal::MetalRenderer;
        Box::new(MetalRenderer::new(window, color))
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
            shadow_buffer: None,
            shadow_index_buffer: None,
            shadow_dirty: true,
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

    // Shadow configuration methods
    pub fn with_shadow(mut self, radius: f32, color: Vec4, offset: Vec2) -> Self {
        self.shadow_radius = radius;
        self.shadow_color = color;
        self.shadow_offset = offset;
        self.shadow_on = true;
        self.shadow_dirty = true;
        self
    }

    pub fn set_shadow(&mut self, radius: f32, color: Vec4, offset: Vec2) {
        self.shadow_radius = radius;
        self.shadow_color = color;
        self.shadow_offset = offset;
        self.shadow_on = true;
        self.shadow_dirty = true;
    }

    pub fn set_shadow_radius(&mut self, radius: f32) {
        self.shadow_radius = radius;
        self.shadow_dirty = true;
    }

    pub fn set_shadow_offset(&mut self, offset: Vec2) {
        self.shadow_offset = offset;
        self.shadow_dirty = true;
    }

    pub fn set_scale(&mut self, scale: Vec2) {
        self.scale = scale;
        self.shadow_dirty = true;
    }

    pub fn enable_shadow(&mut self) {
        self.shadow_on = true;
        self.shadow_dirty = true;
    }

    pub fn disable_shadow(&mut self) {
        self.shadow_on = false;
        self.shadow_dirty = true;
    }

    pub fn toggle_shadow(&mut self) {
        self.shadow_on = !self.shadow_on;
        self.shadow_dirty = true;
    }
}
