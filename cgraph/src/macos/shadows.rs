use glam::{Mat4, Vec2, Vec4};
use metal::MetalLayer;

use crate::{
    metal::{MetalRenderer, Uniforms},
    object::Object,
};

#[repr(C)]
pub struct ShadowUniforms {
    pub offset_x: f32,
    pub offset_y: f32,
    pub radius: f32,
    pub color: Vec4,
    pub enabled: u32,
}

impl Object {
    pub fn make_shadow_uniforms_enabled(&self) -> crate::object::buffer::Buffer<ShadowUniforms> {
        let shadow_uniforms = ShadowUniforms {
            offset_x: self.shadow_offset.x,
            offset_y: self.shadow_offset.y,
            radius: self.shadow_radius,
            color: self.shadow_color,
            enabled: 1, // Enable shadow rendering
        };
        crate::object::buffer::Buffer::new(vec![shadow_uniforms])
    }

    pub fn make_shadow_uniforms_disabled(&self) -> crate::object::buffer::Buffer<ShadowUniforms> {
        let shadow_uniforms = ShadowUniforms {
            offset_x: 0.0,
            offset_y: 0.0,
            radius: 0.0,
            color: Vec4::new(0.0, 0.0, 0.0, 0.0),
            enabled: 0, // Disable shadow rendering for main object
        };
        crate::object::buffer::Buffer::new(vec![shadow_uniforms])
    }

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

impl MetalRenderer {
    pub fn create_shadow_object(&self, object: &Object) -> Object {
        let mut shadow_object = object.clone();

        shadow_object.position.x += object.shadow_offset.x;
        shadow_object.position.y += object.shadow_offset.y;

        let shadow_scale_factor = 1.0
            + (object.shadow_radius
                / object
                    .original_pixel_size
                    .x
                    .min(object.original_pixel_size.y))
                * 0.5;
        shadow_object.scale.x *= shadow_scale_factor;
        shadow_object.scale.y *= shadow_scale_factor;

        for vertex in &mut shadow_object.vertices {
            vertex.z_index = object.vertices[0].z_index - 0.1;
        }

        shadow_object
    }
}
