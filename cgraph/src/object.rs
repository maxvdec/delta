use glam::{Vec2, Vec4};

use crate::object::buffer::Buffer;

#[cfg(target_os = "macos")]
use crate::macos::image::Image;

#[repr(C)]
#[derive(Debug)]
pub struct Vertex {
    pub position: Vec2,
    pub color: Vec4,
    pub z_index: f32,
    pub uv: Vec2,
}

impl Clone for Vertex {
    fn clone(&self) -> Self {
        Vertex {
            position: self.position,
            color: self.color,
            z_index: self.z_index,
            uv: self.uv,
        }
    }
}

impl Vertex {
    pub fn default() -> Self {
        Vertex {
            position: Vec2::new(0.0, 0.0),
            color: Vec4::new(1.0, 1.0, 1.0, 1.0),
            z_index: 0.0,
            uv: Vec2::new(0.0, 0.0),
        }
    }

    pub fn new(x: f32, y: f32, z_index: f32, color: Vec4, uv: Vec2) -> Self {
        Vertex {
            position: Vec2::new(x, y),
            color,
            z_index,
            uv: uv,
        }
    }
}

#[derive(Debug)]
pub struct Object {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,

    pub buffer: Buffer<Vertex>,
    pub index_buffer: Buffer<u32>,

    pub position: Vec2,
    pub scale: Vec2,
    pub original_pixel_size: Vec2,
    pub rotation: f32,
    pub corner_radius: f32,

    #[cfg(target_os = "macos")]
    pub texture: Option<Image>,
    pub use_texture: bool,
}

impl Object {
    pub fn update_buffer(&mut self) {
        self.buffer.update(self.vertices.clone());
        self.index_buffer.update(self.indices.clone());
    }

    pub fn get_buffer(&self) -> &Buffer<Vertex> {
        &self.buffer
    }

    pub fn get_index_buffer(&self) -> &Buffer<u32> {
        &self.index_buffer
    }
}

pub mod buffer;
pub mod primitives;
