use std::path::PathBuf;

use font_kit::{handle::Handle, source::SystemSource};

pub trait Font: std::fmt::Debug {}

#[derive(Debug, Clone)]
pub struct ByteFont {
    pub data: Vec<u8>,
}

impl Font for ByteFont {}

#[derive(Default, Debug, Clone)]
pub struct PathFont {
    pub path: PathBuf,
}

impl Font for PathFont {}

pub fn load_font(path: &str) -> Result<Box<dyn Font>, Box<dyn std::error::Error>> {
    let font = PathFont {
        path: PathBuf::from(path),
    };
    if !font.path.exists() {
        return Err(format!("Font file not found: {path}").into());
    }
    Ok(Box::new(font))
}

pub fn load_system_font(name: &str) -> Result<Box<dyn Font>, Box<dyn std::error::Error>> {
    let system_source = SystemSource::new();
    let handle = system_source
        .select_by_postscript_name(name)
        .map_err(|_| format!("Font '{name}' not found"))?;
    match handle {
        Handle::Memory { bytes, .. } => {
            let font = ByteFont {
                data: bytes.to_vec(),
            };
            Ok(Box::new(font))
        }
        Handle::Path { path, .. } => {
            let font = PathFont { path };
            Ok(Box::new(font))
        }
    }
}
