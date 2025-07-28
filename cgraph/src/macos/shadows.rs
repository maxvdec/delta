use glam::{Mat4, Vec2, Vec4};
use metal::MetalLayer;

use crate::{
    macos::metal::Uniforms,
    object::{Object, Vertex},
};

#[repr(C)]
/// Represents the uniforms used for shadow rendering in Metal.
pub struct ShadowUniforms {
    /// The x offset for the shadow position.
    pub offset_x: f32,
    /// The y offset for the shadow position.
    pub offset_y: f32,
    /// The radius of the shadow.
    pub radius: f32,
    /// The color of the shadow.
    pub color: Vec4,
    /// Whether the shadow is enabled.
    pub enabled: u32,
}

impl Object {
    /// Creates the uniforms for shadow rendering.
    pub fn make_shadow_uniforms_enabled(&self) -> crate::object::buffer::Buffer<ShadowUniforms> {
        let shadow_uniforms = ShadowUniforms {
            offset_x: self.shadow_offset.x,
            offset_y: self.shadow_offset.y,
            radius: self.shadow_radius,
            color: self.shadow_color,
            enabled: true as u32, // Enable shadow rendering
        };
        crate::object::buffer::Buffer::new(vec![shadow_uniforms])
    }

    /// Creates the uniforms for shadow rendering with disabled state.
    pub fn make_shadow_uniforms_disabled(&self) -> crate::object::buffer::Buffer<ShadowUniforms> {
        let shadow_uniforms = ShadowUniforms {
            offset_x: 0.0,
            offset_y: 0.0,
            radius: 0.0,
            color: Vec4::new(0.0, 0.0, 0.0, 0.0),
            enabled: false as u32, // Disable shadow rendering for main object
        };
        crate::object::buffer::Buffer::new(vec![shadow_uniforms])
    }

    /// Creates the uniforms for shadow rendering with offset position.
    pub fn make_shadow_position_uniforms(
        &self,
        layer: &MetalLayer,
    ) -> crate::object::buffer::Buffer<Uniforms> {
        // Create uniforms for shadow with offset position
        let shadow_position = Vec2::new(
            self.position.x + self.shadow_offset.x,
            self.position.y + self.shadow_offset.y,
        );

        let translation = Mat4::from_translation(shadow_position.extend(0.0));
        let scale = Mat4::from_scale(Vec2::new(self.scale.x, self.scale.y).extend(1.0));
        let rotation = Mat4::from_rotation_z(self.rotation);

        let width = layer.drawable_size().width as f32;
        let height = layer.drawable_size().height as f32;
        let projection = Mat4::orthographic_rh(0.0, width, height, 0.0, -100.0, 100.0);

        let rect_size = Vec2::new(
            self.original_pixel_size.x * self.scale.x,
            self.original_pixel_size.y * self.scale.y,
        );

        let model_matrix = translation * rotation * scale;
        let uniforms = Uniforms {
            rect_position: shadow_position,
            rect_size,
            corner_radius: self.corner_radius,
            model_matrix,
            projection_matrix: projection,
            use_texture: false as u32, // Shadows don't use textures
            shadow_radius: 0.0,        // Not used for shadow objects
            shadow_color: Vec4::new(0.0, 0.0, 0.0, 0.0), // Not used for shadow objects
        };

        crate::object::buffer::Buffer::new(vec![uniforms])
    }
}

impl Object {
    /// Creates the geometry for the shadow based on the object's size and shadow properties.
    pub fn create_shadow_geometry(&self) -> (Vec<Vertex>, Vec<u32>) {
        let expansion = self.shadow_radius;

        let original_width = self.original_pixel_size.x * self.scale.x;
        let original_height = self.original_pixel_size.y * self.scale.y;

        let expanded_width = original_width + (expansion * 2.0);
        let expanded_height = original_height + (expansion * 2.0);

        let half_expanded_width = expanded_width * 0.5;
        let half_expanded_height = expanded_height * 0.5;

        let z_index = self.vertices[0].z_index - 0.1;

        let vertices = vec![
            // Bottom-left
            Vertex {
                position: Vec2::new(-half_expanded_width, -half_expanded_height) + self.position,
                color: Vec4::new(1.0, 1.0, 1.0, 1.0), // Color doesn't matter for shadows
                z_index,                              // Render slightly behind main object
                uv: Vec2::new(0.0, 0.0),
            },
            // Bottom-right
            Vertex {
                position: Vec2::new(half_expanded_width, -half_expanded_height) + self.position,
                color: Vec4::new(1.0, 1.0, 1.0, 1.0),
                z_index,
                uv: Vec2::new(1.0, 0.0),
            },
            // Top-right
            Vertex {
                position: Vec2::new(half_expanded_width, half_expanded_height) + self.position,
                color: Vec4::new(1.0, 1.0, 1.0, 1.0),
                z_index,
                uv: Vec2::new(1.0, 1.0),
            },
            // Top-left
            Vertex {
                position: Vec2::new(-half_expanded_width, half_expanded_height) + self.position,
                color: Vec4::new(1.0, 1.0, 1.0, 1.0),
                z_index,
                uv: Vec2::new(0.0, 1.0),
            },
        ];

        // Indices for two triangles
        let indices = vec![0, 1, 2, 0, 2, 3];

        (vertices, indices)
    }

    /// Returns the shadow buffer, creating it if necessary.
    pub fn get_shadow_buffer(&mut self) -> &crate::object::buffer::Buffer<Vertex> {
        if self.shadow_buffer.is_none() || self.shadow_dirty {
            let (vertices, _) = self.create_shadow_geometry();
            self.shadow_buffer = Some(crate::object::buffer::Buffer::new(vertices));
            self.shadow_dirty = false;
        }
        self.shadow_buffer.as_ref().unwrap()
    }

    /// Returns the shadow index buffer, creating it if necessary.
    pub fn get_shadow_index_buffer(&mut self) -> &crate::object::buffer::Buffer<u32> {
        if self.shadow_index_buffer.is_none() {
            let (_, indices) = self.create_shadow_geometry();
            self.shadow_index_buffer = Some(crate::object::buffer::Buffer::new(indices));
        }
        self.shadow_index_buffer.as_ref().unwrap()
    }

    /// Creates the shadow position uniforms for rendering.
    pub fn make_shadow_position_uniforms_expanded(
        &self,
        layer: &MetalLayer,
    ) -> crate::object::buffer::Buffer<Uniforms> {
        let shadow_position = Vec2::new(
            self.position.x + self.shadow_offset.x,
            self.position.y + self.shadow_offset.y,
        );

        let translation = Mat4::from_translation(shadow_position.extend(0.0));
        let scale = Mat4::from_scale(Vec2::new(self.scale.x, self.scale.y).extend(1.0));
        let rotation = Mat4::from_rotation_z(self.rotation);

        let width = layer.drawable_size().width as f32;
        let height = layer.drawable_size().height as f32;
        let projection = Mat4::orthographic_rh(0.0, width, height, 0.0, -100.0, 100.0);

        let rect_size = Vec2::new(
            self.original_pixel_size.x * self.scale.x,
            self.original_pixel_size.y * self.scale.y,
        );

        let model_matrix = translation * rotation * scale;

        let uniforms = Uniforms {
            rect_position: shadow_position,
            rect_size, // Keep original size for SDF
            corner_radius: self.corner_radius,
            model_matrix,
            projection_matrix: projection,
            use_texture: false as u32,
            shadow_radius: 0.0,
            shadow_color: Vec4::new(0.0, 0.0, 0.0, 0.0),
        };

        crate::object::buffer::Buffer::new(vec![uniforms])
    }
}
