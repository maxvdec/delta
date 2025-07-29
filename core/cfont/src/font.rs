/// Module to load and shape fonts.
pub mod load;
/// Module to produce text geometries from shaped text.
pub mod shape;
/// Module to handle font styles.
pub mod style;

use load::get_system_font_with_style;
use shape::{TextGeometry, TextTransform, produce_styled_text};
use std::error::Error;
use style::TextStyle;

/// High-level function to render styled text with a system font.
///
/// # Arguments
/// * `family_name` - The font family name (e.g., "Arial", "Times New Roman")
/// * `text` - The text to render
/// * `transform` - Text transform containing size, position, and canvas info
///
/// # Returns
/// A `TextGeometry` that can be used for rendering, or an error if font loading fails.
pub fn render_text(
    family_name: &str,
    text: &str,
    transform: TextTransform,
) -> Result<TextGeometry, Box<dyn Error>> {
    let font = get_system_font_with_style(family_name, &transform.style)?;
    let mut geometry = produce_styled_text(font, text, &transform.style)?;

    // Apply the transform to position and scale the text
    let font_units_per_em = 1000.0; // Default value, could be extracted from font metrics
    geometry.transform_to_canvas(transform.clone(), font_units_per_em);
    geometry.normalize_to_canvas(transform.canvas_size);

    Ok(geometry)
}

/// Convenience function to create simple text geometry with default styling.
pub fn render_simple_text(
    family_name: &str,
    text: &str,
    font_size: f32,
    position: [f32; 2],
    canvas_size: [f32; 2],
) -> Result<TextGeometry, Box<dyn Error>> {
    let transform = TextTransform {
        font_size,
        position,
        canvas_size,
        style: TextStyle::new(),
    };

    render_text(family_name, text, transform)
}
