use std::error::Error;

use crate::font::load::Font;
use crate::font::style::TextStyle;
use font_kit::outline::OutlineSink;
use lyon::{
    path::Path,
    tessellation::{BuffersBuilder, FillOptions, FillTessellator, FillVertex, VertexBuffers},
};
use pathfinder_geometry::line_segment::LineSegment2F;
use pathfinder_geometry::vector::Vector2F;
use rustybuzz::{Face, UnicodeBuffer};

#[derive(Debug)]
/// Represents a vertex in the text geometry.
pub struct TextVertex {
    /// The position of the vertex in 2D space.
    pub position: [f32; 2],
}

#[derive(Debug)]
/// Represents the geometry of a text shape.
pub struct TextGeometry {
    /// The vertices of the text geometry.
    pub vertices: Vec<TextVertex>,
    /// The indices of the vertices for indexed rendering.
    pub indices: Vec<u16>,
}

#[derive(Debug, Clone)]
/// Represents a transformation for text rendering.
pub struct TextTransform {
    /// Desired font size in pixels.
    pub font_size: f32, // Desired font size in pixels
    /// Position in pixels [x, y]
    pub position: [f32; 2], // Position in pixels [x, y]
    /// Canvas size in pixels [width, height]
    pub canvas_size: [f32; 2], // Canvas size in pixels [width, height]
    /// Text styling properties
    pub style: TextStyle,
}

impl TextGeometry {
    /// Creates a new `TextGeometry` from the given vertices and indices.
    pub fn bounding_box(&self) -> (f32, f32, f32, f32) {
        if self.vertices.is_empty() {
            return (0.0, 0.0, 0.0, 0.0);
        }

        let mut min_x = f32::INFINITY;
        let mut max_x = f32::NEG_INFINITY;
        let mut min_y = f32::INFINITY;
        let mut max_y = f32::NEG_INFINITY;

        for vertex in &self.vertices {
            let [x, y] = vertex.position;
            min_x = min_x.min(x);
            max_x = max_x.max(x);
            min_y = min_y.min(y);
            max_y = max_y.max(y);
        }

        (min_x, min_y, max_x, max_y)
    }

    /// Transforms the text geometry to fit within the specified canvas size.
    pub fn transform_to_canvas(&mut self, transform: TextTransform, font_units_per_em: f32) {
        let (min_x, _min_y, _max_x, max_y) = self.bounding_box();

        let font_scale = transform.font_size / font_units_per_em;

        for vertex in &mut self.vertices {
            let [x, y] = vertex.position;

            let pixel_x = x * font_scale;
            let pixel_y = y * font_scale;

            let local_x = pixel_x - (min_x * font_scale);
            let local_y = (max_y * font_scale) - pixel_y;

            vertex.position = [local_x, local_y];
        }
    }

    pub fn normalize_to_canvas(&mut self, canvas_size: [f32; 2]) {
        for vertex in &mut self.vertices {
            vertex.position[0] /= canvas_size[0];
            vertex.position[1] /= canvas_size[1];
        }
    }

    pub fn pixel_dimensions(&self) -> (f32, f32) {
        let (min_x, min_y, max_x, max_y) = self.bounding_box();
        (max_x - min_x, max_y - min_y)
    }
}

struct PathBuilderSink {
    builder: lyon::path::Builder,
}

impl PathBuilderSink {
    fn new() -> Self {
        Self {
            builder: Path::builder(),
        }
    }

    fn build(self) -> Path {
        self.builder.build()
    }
}

impl OutlineSink for PathBuilderSink {
    fn move_to(&mut self, to: Vector2F) {
        self.builder.begin(lyon::math::Point::new(to.x(), to.y()));
    }

    fn line_to(&mut self, to: Vector2F) {
        self.builder.line_to(lyon::math::Point::new(to.x(), to.y()));
    }

    fn quadratic_curve_to(&mut self, ctrl: Vector2F, to: Vector2F) {
        self.builder.quadratic_bezier_to(
            lyon::math::Point::new(ctrl.x(), ctrl.y()),
            lyon::math::Point::new(to.x(), to.y()),
        );
    }

    fn cubic_curve_to(&mut self, ctrl: LineSegment2F, to: Vector2F) {
        self.builder.cubic_bezier_to(
            lyon::math::Point::new(ctrl.from().x(), ctrl.from().y()),
            lyon::math::Point::new(ctrl.to().x(), ctrl.to().y()),
            lyon::math::Point::new(to.x(), to.y()),
        );
    }

    fn close(&mut self) {
        self.builder.end(true);
    }
}

/// Extracts font metrics for better character spacing.
fn get_font_metrics(font: &font_kit::font::Font) -> (f32, f32, f32) {
    let metrics = font.metrics();
    let units_per_em = metrics.units_per_em as f32;
    let line_height = metrics.ascent + metrics.descent + metrics.line_gap;
    let ascent = metrics.ascent;

    (units_per_em, line_height, ascent)
}

/// Calculates font-specific spacing multiplier based on font characteristics.
fn calculate_spacing_multiplier(font_family_name: &str) -> f32 {
    // Font-specific spacing adjustments for fonts known to have spacing issues
    match font_family_name.to_lowercase().as_str() {
        "papyrus" => 1.4,             // Papyrus needs more spacing due to decorative nature
        "chalkduster" => 1.3,         // Similar decorative font
        "herculanum" => 1.3,          // Another decorative font
        "bradley hand" => 1.2,        // Handwritten style fonts
        "marker felt" => 1.2,         // Marker style fonts
        "luminari" => 1.3,            // Fantasy/decorative fonts
        "zapfino" => 1.5,             // Script fonts with lots of flourishes
        "snell roundhand" => 1.3,     // Script fonts
        "american typewriter" => 1.1, // Typewriter fonts can be tight
        _ => {
            // For unknown fonts, apply a small increase if they appear to be decorative
            if font_family_name.to_lowercase().contains("script")
                || font_family_name.to_lowercase().contains("hand")
                || font_family_name.to_lowercase().contains("brush")
                || font_family_name.to_lowercase().contains("calligraphy")
            {
                1.2
            } else {
                1.0 // Standard spacing for regular fonts
            }
        }
    }
}

/// Produces a `TextGeometry` from the given font and text string with font-aware spacing.
pub fn produce_text(font: Font, text: &str) -> Result<TextGeometry, Box<dyn Error>> {
    produce_text_with_family_name(font, text, "")
}

/// Produces a `TextGeometry` from the given font and text string with font family name for spacing adjustments.
pub fn produce_text_with_family_name(
    font: Font,
    text: &str,
    font_family_name: &str,
) -> Result<TextGeometry, Box<dyn Error>> {
    let cfont_font = font.clone();
    let face: Result<Face<'_>, Box<dyn Error>> =
        Face::from_slice(&font.bytes, 0).ok_or_else(|| "Value was none".into());

    let core_font = cfont_font.core_font.unwrap();

    let mut buffer = UnicodeBuffer::new();
    buffer.push_str(text);
    let glyph_buffer = rustybuzz::shape(&face?, &[], buffer);

    let mut global_geometry: VertexBuffers<[f32; 2], u16> = VertexBuffers::new();
    let mut tess = FillTessellator::new();

    // Extract font metrics for better spacing calculations
    let (units_per_em, _line_height, _ascent) = get_font_metrics(&core_font);

    // Calculate spacing multiplier based on font characteristics
    let spacing_multiplier = calculate_spacing_multiplier(font_family_name);

    let mut cursor_x = 0.0f32;

    for glyph in glyph_buffer
        .glyph_infos()
        .iter()
        .zip(glyph_buffer.glyph_positions())
    {
        let (info, pos) = glyph;
        let gid = info.glyph_id;
        let x_offset = pos.x_offset as f32;
        let y_offset = pos.y_offset as f32;
        let mut x_advance = pos.x_advance as f32;

        // Apply font-specific spacing adjustments
        if spacing_multiplier != 1.0 {
            x_advance *= spacing_multiplier;

            // For decorative fonts, also add a small minimum spacing
            if spacing_multiplier > 1.2 {
                let min_spacing = units_per_em * 0.05; // 5% of em size as minimum spacing
                x_advance = x_advance.max(min_spacing);
            }
        }

        let mut sink = PathBuilderSink::new();
        core_font.outline(gid, font_kit::hinting::HintingOptions::None, &mut sink)?;
        let path = sink.build();

        tess.tessellate_path(
            &path,
            &FillOptions::default(),
            &mut BuffersBuilder::new(&mut global_geometry, |v: FillVertex| {
                [
                    v.position().x + cursor_x + x_offset,
                    v.position().y + y_offset,
                ]
            }),
        )?;

        cursor_x += x_advance;
    }

    let vertices: Vec<TextVertex> = global_geometry
        .vertices
        .into_iter()
        .map(|pos| TextVertex { position: pos })
        .collect();

    Ok(TextGeometry {
        vertices,
        indices: global_geometry.indices,
    })
}

/// Produces a `TextGeometry` from the given font, text string, and style.
/// This version applies style-specific transformations like bold stroke width.
pub fn produce_styled_text(
    font: Font,
    text: &str,
    style: &TextStyle,
) -> Result<TextGeometry, Box<dyn Error>> {
    produce_styled_text_with_family_name(font, text, style, "")
}

/// Produces a `TextGeometry` from the given font, text string, style, and font family name for spacing adjustments.
pub fn produce_styled_text_with_family_name(
    font: Font,
    text: &str,
    style: &TextStyle,
    font_family_name: &str,
) -> Result<TextGeometry, Box<dyn Error>> {
    let mut geometry = produce_text_with_family_name(font, text, font_family_name)?;

    // Apply style-specific modifications
    if style.is_bold() {
        // For bold text, we could apply a stroke effect or use a different tessellation
        // This is a simplified approach - in a real implementation you might:
        // 1. Use a bold variant of the font
        // 2. Apply stroke/outline effects
        // 3. Slightly expand the geometry

        // For now, we'll keep the geometry as-is since we're using font-kit's font selection
        // which should already handle bold weights
    }

    // Note: Italic is typically handled by font selection (using slanted font variants)
    // Underline would be added as additional geometry below the text
    if style.underlined {
        add_underline_geometry(&mut geometry);
    }

    Ok(geometry)
}

/// Adds underline geometry to existing text geometry.
fn add_underline_geometry(geometry: &mut TextGeometry) {
    let (min_x, min_y, max_x, _) = geometry.bounding_box();

    if geometry.vertices.is_empty() {
        return;
    }

    // Calculate underline position and thickness
    let underline_y = min_y - 20.0; // Position below text baseline
    let underline_thickness = 2.0;

    // Create underline rectangle vertices
    let underline_vertices = vec![
        TextVertex {
            position: [min_x, underline_y],
        },
        TextVertex {
            position: [max_x, underline_y],
        },
        TextVertex {
            position: [max_x, underline_y - underline_thickness],
        },
        TextVertex {
            position: [min_x, underline_y - underline_thickness],
        },
    ];

    let base_index = geometry.vertices.len() as u16;

    // Add underline indices (two triangles)
    let underline_indices = vec![
        base_index,
        base_index + 1,
        base_index + 2,
        base_index,
        base_index + 2,
        base_index + 3,
    ];

    // Append to existing geometry
    geometry.vertices.extend(underline_vertices);
    geometry.indices.extend(underline_indices);
}
