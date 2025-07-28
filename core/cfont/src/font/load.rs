use std::{error::Error, path::PathBuf};

use font_kit::{handle::Handle, source::SystemSource};

#[derive(Debug, Default, Clone)]
pub struct Font {
    pub path: Option<PathBuf>,
    pub core_font: Option<font_kit::font::Font>,
    pub stored_in_memory: bool,
    pub bytes: Vec<u8>,
}

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
