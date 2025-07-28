use std::error::Error;

use cfont::font::shape::{TextTransform, produce_text};
use glam::Vec2;

use crate::{
    app::Window,
    object::{Object, Vertex, primitives::Color},
};

/// Represents a font with its core font and transformation properties.
pub struct Font {
    /// The core font used for rendering text.
    pub core_font: cfont::font::load::Font,
    /// The transformation properties for rendering text.
    pub transform: cfont::font::shape::TextTransform,
}

/// Retrieves a system font and creates a `Font` instance with the specified name and size.
pub fn get_font(window: &Window, name: &str, size: f32) -> Result<Font, Box<dyn Error>> {
    let font = cfont::font::load::get_system_font(name)?;
    let transform = TextTransform {
        font_size: size,
        position: [0.0, 0.0],
        canvas_size: [window.width as f32, window.height as f32],
    };
    Ok(Font {
        core_font: font,
        transform,
    })
}

/// Creates a text object with the specified font, text, color, z-index, and position.
pub fn make_text(
    font: Font,
    text: &str,
    color: Color,
    z_index: f32,
    position: Vec2,
) -> Result<Object, Box<dyn Error>> {
    let mut result = produce_text(font.core_font, text)?;

    let font_units_per_em = 1000.0;
    let mut transform = font.transform.clone();
    transform.position = [position.x, position.y];
    result.transform_to_canvas(font.transform, font_units_per_em);

    let mut indices: Vec<u32> = vec![];
    for index in result.indices {
        indices.push(index as u32);
    }

    let mut vertices: Vec<Vertex> = vec![];
    for vertex in result.vertices {
        let vertex = Vertex {
            position: Vec2::new(vertex.position[0], -vertex.position[1]),
            color,
            z_index,
            uv: Vec2::new(0.0, 0.0),
        };
        vertices.push(vertex)
    }

    let mut object = Object::new(vertices, indices);
    object.position = position;
    object.scale = Vec2::new(1.0, 1.0);
    object.original_pixel_size = position;
    object.rotation = 0.0;
    object.corner_radius = 0.0;
    Ok(object)
}
