use glam::{Vec2, Vec4};

use crate::object::buffer::Buffer;
use crate::object::primitives::Color;
use crate::object::{Object, Vertex};

/// Renderer trait for rendering graphics objects.
pub trait Renderer {
    /// Creates a new renderer instance.
    fn new(window: &winit::window::Window, background_color: Color) -> Self
    where
        Self: Sized;
    /// Initializes the renderer.
    fn render(&mut self, window: &winit::window::Window);
    /// Resizes the renderer to the specified width and height.
    fn resize(&mut self, width: f64, height: f64);
    /// Destroys the renderer and releases resources.
    fn destroy(&self);
    /// Adds an object to the renderer.
    fn add_object(&mut self, object: Object);
    /// Clears all objects from the renderer.
    fn clear(&mut self);
    /// Renders all objects in the renderer.
    fn as_any(&self) -> &dyn std::any::Any;
    #[allow(dead_code)]
    /// Returns a mutable reference to the renderer as a dynamic type.
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any;
    /// Sets the background color of the renderer.
    fn set_background_color(&mut self, background_color: Color);
}

/// Creates a new renderer based on the platform.
pub fn create_renderer(window: &winit::window::Window, color: Color) -> Box<dyn Renderer> {
    if cfg!(target_os = "macos") {
        use crate::macos::metal::MetalRenderer;
        Box::new(MetalRenderer::new(window, color))
    } else {
        panic!("Unsupported platform");
    }
}

impl Object {
    /// Creates a new `Object` with the specified vertices and indices.
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

    /// Adds a vertex to the object.
    pub fn add_vertex(&mut self, vertex: Vertex) {
        self.vertices.push(vertex);
    }

    /// Adds an index to the object.
    pub fn add_index(&mut self, index: u32) {
        self.indices.push(index);
    }

    #[cfg(target_os = "macos")]
    /// Sets the texture for the object.
    pub fn with_texture(mut self, texture: crate::macos::image::Image) -> Self {
        self.texture = Some(texture);
        self.use_texture = true;
        self
    }

    /// Sets whether the object uses a texture.
    pub fn set_use_texture(&mut self, use_texture: bool) {
        self.use_texture = use_texture;
    }

    /// Renders the object with a shadow.
    pub fn with_shadow(mut self, radius: f32, color: Vec4, offset: Vec2) -> Self {
        self.shadow_radius = radius;
        self.shadow_color = color;
        self.shadow_offset = offset;
        self.shadow_on = true;
        self.shadow_dirty = true;
        self
    }

    /// Updates the shadow properties of the object.
    pub fn set_shadow(&mut self, radius: f32, color: Vec4, offset: Vec2) {
        self.shadow_radius = radius;
        self.shadow_color = color;
        self.shadow_offset = offset;
        self.shadow_on = true;
        self.shadow_dirty = true;
    }

    /// Sets the corner radius for the object.
    pub fn set_shadow_radius(&mut self, radius: f32) {
        self.shadow_radius = radius;
        self.shadow_dirty = true;
    }

    /// Sets the shadow color for the object.
    pub fn set_shadow_offset(&mut self, offset: Vec2) {
        self.shadow_offset = offset;
        self.shadow_dirty = true;
    }

    /// Sets the position of the object.
    pub fn set_scale(&mut self, scale: Vec2) {
        self.scale = scale;
        self.shadow_dirty = true;
    }

    /// Sets the rotation of the object.
    pub fn enable_shadow(&mut self) {
        self.shadow_on = true;
        self.shadow_dirty = true;
    }

    /// Disables the shadow for the object.
    pub fn disable_shadow(&mut self) {
        self.shadow_on = false;
        self.shadow_dirty = true;
    }

    /// Toggles the shadow state of the object.
    pub fn toggle_shadow(&mut self) {
        self.shadow_on = !self.shadow_on;
        self.shadow_dirty = true;
    }
}
