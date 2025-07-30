use std::error::Error;

use cfont::font::{
    load::get_system_font_with_style,
    shape::{TextTransform, produce_styled_text, produce_text},
    style::{FontWeight, TextStyle},
};
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

/// A friendly API for text styling in Core Graphics.
#[derive(Debug, Clone)]
pub struct TextStyleBuilder {
    style: TextStyle,
    font_family: String,
    font_size: f32,
}

impl TextStyleBuilder {
    /// Creates a new text style builder with default settings.
    pub fn new(font_family: &str, font_size: f32) -> Self {
        Self {
            style: TextStyle::new(),
            font_family: font_family.to_string(),
            font_size,
        }
    }

    /// Makes the text bold.
    pub fn bold(mut self) -> Self {
        self.style = self.style.with_weight(FontWeight::Bold);
        self
    }

    /// Makes the text italic.
    pub fn italic(mut self) -> Self {
        self.style = self.style.with_italic(true);
        self
    }

    /// Makes the text underlined.
    pub fn underlined(mut self) -> Self {
        self.style = self.style.with_underlined(true);
        self
    }

    /// Sets a specific font weight.
    pub fn weight(mut self, weight: FontWeight) -> Self {
        self.style = self.style.with_weight(weight);
        self
    }

    /// Sets the font weight to thin.
    pub fn thin(mut self) -> Self {
        self.style = self.style.with_weight(FontWeight::Thin);
        self
    }

    /// Sets the font weight to light.
    pub fn light(mut self) -> Self {
        self.style = self.style.with_weight(FontWeight::Light);
        self
    }

    /// Sets the font weight to medium.
    pub fn medium(mut self) -> Self {
        self.style = self.style.with_weight(FontWeight::Medium);
        self
    }

    /// Sets the font weight to semi-bold.
    pub fn semi_bold(mut self) -> Self {
        self.style = self.style.with_weight(FontWeight::SemiBold);
        self
    }

    /// Sets the font weight to extra bold.
    pub fn extra_bold(mut self) -> Self {
        self.style = self.style.with_weight(FontWeight::ExtraBold);
        self
    }

    /// Sets the font weight to black (heaviest).
    pub fn black(mut self) -> Self {
        self.style = self.style.with_weight(FontWeight::Black);
        self
    }

    /// Changes the font size.
    pub fn size(mut self, size: f32) -> Self {
        self.font_size = size;
        self
    }

    /// Changes the font family.
    pub fn family(mut self, family: &str) -> Self {
        self.font_family = family.to_string();
        self
    }

    /// Builds the styled font for use with the text rendering system.
    pub fn build_font(self, window: &Window) -> Result<StyledFont, Box<dyn Error>> {
        let core_font = get_system_font_with_style(&self.font_family, &self.style)?;
        let transform = TextTransform {
            font_size: self.font_size,
            position: [0.0, 0.0],
            canvas_size: [window.width as f32, window.height as f32],
            style: self.style,
        };

        Ok(StyledFont {
            core_font,
            transform,
            font_family: self.font_family,
        })
    }
}

/// A styled font that includes both the font data and styling information.
pub struct StyledFont {
    /// The core font used for rendering text.
    pub core_font: cfont::font::load::Font,
    /// The transformation properties for rendering text.
    pub transform: cfont::font::shape::TextTransform,
    /// The font family name.
    pub font_family: String,
}

/// Retrieves a system font and creates a `Font` instance with the specified name and size.
pub fn get_font(window: &Window, name: &str, size: f32) -> Result<Font, Box<dyn Error>> {
    let font = cfont::font::load::get_system_font(name)?;
    let transform = TextTransform {
        font_size: size,
        position: [0.0, 0.0],
        canvas_size: [window.width as f32, window.height as f32],
        style: TextStyle::new(),
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
    result.transform_to_canvas(font.transform, font_units_per_em);

    let mut indices: Vec<u32> = vec![];
    for index in result.indices {
        indices.push(index as u32);
    }

    let mut vertices: Vec<Vertex> = vec![];
    for vertex in result.vertices {
        let vertex = Vertex {
            position: Vec2::new(
                vertex.position[0] + position.x,
                vertex.position[1] + position.y,
            ),
            color,
            z_index,
            uv: Vec2::new(0.0, 0.0),
        };
        vertices.push(vertex)
    }

    let mut object = Object::new(vertices, indices);
    object.position = position;
    object.scale = Vec2::new(1.0, 1.0);
    object.original_pixel_size = Vec2::new(0.0, 0.0); // Will be calculated from text bounds
    object.rotation = 0.0;
    object.corner_radius = 0.0;
    Ok(object)
}

/// Creates a styled text object with enhanced styling support.
pub fn make_styled_text(
    styled_font: StyledFont,
    text: &str,
    color: Color,
    z_index: f32,
    position: Vec2,
) -> Result<Object, Box<dyn Error>> {
    let mut result =
        produce_styled_text(styled_font.core_font, text, &styled_font.transform.style)?;

    let font_units_per_em = 1000.0;
    result.transform_to_canvas(styled_font.transform, font_units_per_em);

    let mut indices: Vec<u32> = vec![];
    for index in result.indices {
        indices.push(index as u32);
    }

    let mut vertices: Vec<Vertex> = vec![];
    for vertex in result.vertices {
        let vertex = Vertex {
            position: Vec2::new(
                vertex.position[0] + position.x,
                vertex.position[1] + position.y,
            ),
            color,
            z_index,
            uv: Vec2::new(0.0, 0.0),
        };
        vertices.push(vertex)
    }

    let mut object = Object::new(vertices, indices);
    object.position = position;
    object.scale = Vec2::new(1.0, 1.0);
    object.original_pixel_size = Vec2::new(0.0, 0.0); // Will be calculated from text bounds
    object.rotation = 0.0;
    object.corner_radius = 0.0;
    Ok(object)
}

// Convenience functions for common text styles

/// Creates bold text with the specified parameters.
pub fn make_bold_text(
    window: &Window,
    font_family: &str,
    font_size: f32,
    text: &str,
    color: Color,
    z_index: f32,
    position: Vec2,
) -> Result<Object, Box<dyn Error>> {
    let styled_font = TextStyleBuilder::new(font_family, font_size)
        .bold()
        .build_font(window)?;

    make_styled_text(styled_font, text, color, z_index, position)
}

/// Creates italic text with the specified parameters.
pub fn make_italic_text(
    window: &Window,
    font_family: &str,
    font_size: f32,
    text: &str,
    color: Color,
    z_index: f32,
    position: Vec2,
) -> Result<Object, Box<dyn Error>> {
    let styled_font = TextStyleBuilder::new(font_family, font_size)
        .italic()
        .build_font(window)?;

    make_styled_text(styled_font, text, color, z_index, position)
}

/// Creates underlined text with the specified parameters.
pub fn make_underlined_text(
    window: &Window,
    font_family: &str,
    font_size: f32,
    text: &str,
    color: Color,
    z_index: f32,
    position: Vec2,
) -> Result<Object, Box<dyn Error>> {
    let styled_font = TextStyleBuilder::new(font_family, font_size)
        .underlined()
        .build_font(window)?;

    make_styled_text(styled_font, text, color, z_index, position)
}

/// Creates bold and italic text with the specified parameters.
pub fn make_bold_italic_text(
    window: &Window,
    font_family: &str,
    font_size: f32,
    text: &str,
    color: Color,
    z_index: f32,
    position: Vec2,
) -> Result<Object, Box<dyn Error>> {
    let styled_font = TextStyleBuilder::new(font_family, font_size)
        .bold()
        .italic()
        .build_font(window)?;

    make_styled_text(styled_font, text, color, z_index, position)
}

/// Creates underlined and italic text with the specified parameters.
pub struct FontArgument {
    pub family: &'static str,
    pub size: f32,
}

/// Creates text with custom weight.
pub fn make_weighted_text(
    window: &Window,
    font: FontArgument,
    weight: FontWeight,
    text: &str,
    color: Color,
    z_index: f32,
    position: Vec2,
) -> Result<Object, Box<dyn Error>> {
    let styled_font = TextStyleBuilder::new(font.family, font.size)
        .weight(weight)
        .build_font(window)?;

    make_styled_text(styled_font, text, color, z_index, position)
}

/// Creates text with the TextStyleBuilder for maximum customization.
///
/// ## Example
/// ```ignore
/// let text_object = make_custom_text(
///     &window,
///     TextStyleBuilder::new("Arial", 24.0)
///         .bold()
///         .italic()
///         .underlined(),
///     "Custom styled text!",
///     Color::white(),
///     1.0,
///     Vec2::new(100.0, 200.0)
/// )?;
/// ```
pub fn make_custom_text(
    window: &Window,
    style_builder: TextStyleBuilder,
    text: &str,
    color: Color,
    z_index: f32,
    position: Vec2,
) -> Result<Object, Box<dyn Error>> {
    let styled_font = style_builder.build_font(window)?;
    make_styled_text(styled_font, text, color, z_index, position)
}
