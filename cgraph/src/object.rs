use crate::object::buffer::Buffer;

#[repr(C)]
pub struct Vertex {
    pub position: [f32; 2],
    pub color: [f32; 4],
    pub z_index: f32,
    _padding: f32,
}

impl Clone for Vertex {
    fn clone(&self) -> Self {
        Vertex {
            position: self.position,
            color: self.color,
            z_index: self.z_index,
            _padding: self._padding,
        }
    }
}

impl Vertex {
    pub fn default() -> Self {
        Vertex {
            position: [0.0, 0.0],
            color: [1.0, 1.0, 1.0, 1.0],
            z_index: 0.0,
            _padding: 0.0,
        }
    }

    pub fn new(x: f32, y: f32, z_index: f32, color: [f32; 4]) -> Self {
        Vertex {
            position: [x, y],
            color,
            z_index,
            _padding: 0.0,
        }
    }
}

pub struct Object {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,

    pub buffer: Buffer<Vertex>,
    pub index_buffer: Buffer<u32>,
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
pub mod quad;
