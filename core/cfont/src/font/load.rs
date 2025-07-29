use std::{error::Error, path::PathBuf};

use crate::font::style::{FontWeight, TextStyle};
use font_kit::{
    handle::Handle,
    properties::{Properties, Style, Weight},
    source::SystemSource,
};

#[derive(Debug, Default, Clone)]
/// Represents a font in the Core Font library.
pub struct Font {
    /// The path to the font file, if loaded from a file.
    pub path: Option<PathBuf>,
    /// The core font object, if loaded successfully.
    pub core_font: Option<font_kit::font::Font>,
    /// Whether the font was loaded from memory.
    pub stored_in_memory: bool,
    /// The raw bytes of the font file.
    pub bytes: Vec<u8>,
}

/// Retrieves a system font by its PostScript name.
pub fn get_system_font(name: &str) -> Result<Font, Box<dyn Error>> {
    let source = SystemSource::new();
    let handle = source.select_by_postscript_name(name)?;
    let loaded_in_mem: bool;
    let mut outer_path: Option<PathBuf> = None;
    let font_bytes: Vec<u8>;
    match handle.clone() {
        Handle::Path { path, .. } => {
            loaded_in_mem = false;
            outer_path = Some(path.clone());
            font_bytes = std::fs::read(path).unwrap();
        }
        Handle::Memory { bytes, .. } => {
            loaded_in_mem = true;
            font_bytes = bytes.to_vec();
        }
    }
    let font = handle.load()?;
    Ok(Font {
        path: outer_path,
        core_font: Some(font),
        stored_in_memory: loaded_in_mem,
        bytes: font_bytes,
    })
}

/// Retrieves a system font by family name with specific style properties.
pub fn get_system_font_with_style(
    family_name: &str,
    style: &TextStyle,
) -> Result<Font, Box<dyn Error>> {
    let source = SystemSource::new();

    // Convert TextStyle to font-kit Properties
    let properties = Properties {
        style: if style.italic {
            Style::Italic
        } else {
            Style::Normal
        },
        weight: match style.weight {
            FontWeight::Thin => Weight::THIN,
            FontWeight::ExtraLight => Weight::EXTRA_LIGHT,
            FontWeight::Light => Weight::LIGHT,
            FontWeight::Normal => Weight::NORMAL,
            FontWeight::Medium => Weight::MEDIUM,
            FontWeight::SemiBold => Weight::SEMIBOLD,
            FontWeight::Bold => Weight::BOLD,
            FontWeight::ExtraBold => Weight::EXTRA_BOLD,
            FontWeight::Black => Weight::BLACK,
        },
        stretch: font_kit::properties::Stretch::NORMAL,
    };

    let handle = source.select_best_match(
        &[font_kit::family_name::FamilyName::Title(
            family_name.to_string(),
        )],
        &properties,
    )?;

    let loaded_in_mem: bool;
    let mut outer_path: Option<PathBuf> = None;
    let font_bytes: Vec<u8>;

    match handle.clone() {
        Handle::Path { path, .. } => {
            loaded_in_mem = false;
            outer_path = Some(path.clone());
            font_bytes = std::fs::read(path)?;
        }
        Handle::Memory { bytes, .. } => {
            loaded_in_mem = true;
            font_bytes = bytes.to_vec();
        }
    }

    let font = handle.load()?;
    Ok(Font {
        path: outer_path,
        core_font: Some(font),
        stored_in_memory: loaded_in_mem,
        bytes: font_bytes,
    })
}
