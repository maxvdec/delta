use glam::{Vec2, Vec4};

use crate::object::buffer::Buffer;

#[cfg(target_os = "macos")]
use crate::macos::image::Image;

#[repr(C)]
#[derive(Debug)]
/// Represents a vertex in the graphics object.
pub struct Vertex {
    /// The position of the vertex in 2D space.
    pub position: Vec2,
    /// The color of the vertex.
    pub color: Vec4,
    /// The z-index of the vertex for rendering order.
    pub z_index: f32,
    /// The UV coordinates for texture mapping.
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
    /// Creates a default vertex with position (0, 0), color white, z-index 0, and UV coordinates (0, 0).
    pub fn create_default() -> Self {
        Vertex {
            position: Vec2::new(0.0, 0.0),
            color: Vec4::new(1.0, 1.0, 1.0, 1.0),
            z_index: 0.0,
            uv: Vec2::new(0.0, 0.0),
        }
    }

    /// Creates a new vertex with the specified position, color, z-index, and UV coordinates.
    pub fn new(x: f32, y: f32, z_index: f32, color: Vec4, uv: Vec2) -> Self {
        Vertex {
            position: Vec2::new(x, y),
            color,
            z_index,
            uv,
        }
    }
}

#[derive(Debug)]
/// Represents a graphics object that can be rendered.
pub struct Object {
    /// The vertices of the object.
    pub vertices: Vec<Vertex>,
    /// The indices of the vertices for indexed rendering.
    pub indices: Vec<u32>,

    /// The buffer containing the vertex data.
    pub buffer: Buffer<Vertex>,
    /// The buffer containing the index data.
    pub index_buffer: Buffer<u32>,

    /// The position of the object in 2D space.
    pub position: Vec2,
    /// The scale of the object in 2D space.
    pub scale: Vec2,
    /// The original pixel size of the object.
    pub original_pixel_size: Vec2,
    /// The rotation of the object in radians.
    pub rotation: f32,
    /// The corner radius for rounded corners.
    pub corner_radius: f32,
    /// The radius of the shadow.
    pub shadow_radius: f32,
    /// The color of the shadow.
    pub shadow_color: Vec4,
    /// The offset of the shadow.
    pub shadow_offset: Vec2,
    /// Whether the shadow is enabled.
    pub shadow_on: bool,

    /// The buffer for shadow vertices.
    pub shadow_buffer: Option<Buffer<Vertex>>,
    /// The buffer for shadow indices.
    pub shadow_index_buffer: Option<Buffer<u32>>,
    /// Whether the shadow data is dirty and needs to be updated.
    pub shadow_dirty: bool,

    #[cfg(target_os = "macos")]
    /// The texture associated with the object, if any.
    pub texture: Option<Image>,
    /// Whether the object uses a texture.
    pub use_texture: bool,
}

impl Clone for Object {
    fn clone(&self) -> Self {
        Object {
            vertices: self.vertices.clone(),
            indices: self.indices.clone(),
            buffer: self.buffer.clone(),
            index_buffer: self.index_buffer.clone(),
            position: self.position,
            scale: self.scale,
            original_pixel_size: self.original_pixel_size,
            rotation: self.rotation,
            corner_radius: self.corner_radius,
            shadow_radius: self.shadow_radius,
            shadow_color: self.shadow_color,
            shadow_offset: self.shadow_offset,
            shadow_on: self.shadow_on,
            shadow_buffer: self.shadow_buffer.clone(),
            shadow_index_buffer: self.shadow_index_buffer.clone(),
            shadow_dirty: self.shadow_dirty,
            #[cfg(target_os = "macos")]
            texture: self.texture.clone(),
            use_texture: self.use_texture,
        }
    }
}

impl Object {
    /// Creates a new Object with the given vertices and indices.
    pub fn update_buffer(&mut self) {
        self.buffer.update(self.vertices.clone());
        self.index_buffer.update(self.indices.clone());
    }

    /// Adds a vertex to the object.
    pub fn get_buffer(&self) -> &Buffer<Vertex> {
        &self.buffer
    }

    /// Adds a vertex to the object.
    pub fn get_index_buffer(&self) -> &Buffer<u32> {
        &self.index_buffer
    }
}

/// Buffer module for managing vertex and index data.
pub mod buffer;
/// Curves logic and implementations.
pub mod curve;
/// Primitives module for creating advanced shapes.
pub mod curve_primitives;
/// Primitives module for creating common shapes.
pub mod primitives;
