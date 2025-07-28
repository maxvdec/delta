use metal::*;
use std::path::Path;

#[derive(Debug)]
/// Represents an image in the Metal graphics system.
pub struct Image {
    /// The source path of the image.
    pub source: String,
    /// The Metal texture representing the image.
    pub texture: metal::Texture,
    /// The width of the image in pixels.
    pub width: u32,
    /// The height of the image in pixels.
    pub height: u32,
}

impl Image {
    /// Creates a new Image from a file path.
    pub fn new(source: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let device = metal::Device::system_default().expect("No Metal device found");

        let img = image::open(Path::new(source))?;
        let rgba_img = img.to_rgba8();
        let width = rgba_img.width();
        let height = rgba_img.height();
        let data = rgba_img.as_raw();

        let texture_descriptor = TextureDescriptor::new();
        texture_descriptor.set_width(width as u64);
        texture_descriptor.set_height(height as u64);
        texture_descriptor.set_pixel_format(MTLPixelFormat::RGBA8Unorm);
        texture_descriptor.set_texture_type(MTLTextureType::D2);
        texture_descriptor.set_storage_mode(MTLStorageMode::Managed);
        texture_descriptor.set_usage(MTLTextureUsage::ShaderRead);

        let texture = device.new_texture(&texture_descriptor);

        let region = MTLRegion {
            origin: MTLOrigin { x: 0, y: 0, z: 0 },
            size: MTLSize {
                width: width as u64,
                height: height as u64,
                depth: 1,
            },
        };

        texture.replace_region(
            region,
            0,
            data.as_ptr() as *const std::ffi::c_void,
            (width * 4) as u64, // 4 bytes per pixel (RGBA)
        );

        Ok(Image {
            source: source.to_string(),
            texture,
            width,
            height,
        })
    }

    /// Creates a new Image from a file path using a specific Metal device.
    pub fn new_from_device(
        source: &str,
        device: &metal::Device,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let img = image::open(Path::new(source))?;
        let rgba_img = img.to_rgba8();
        let width = rgba_img.width();
        let height = rgba_img.height();
        let data = rgba_img.as_raw();

        let texture_descriptor = TextureDescriptor::new();
        texture_descriptor.set_width(width as u64);
        texture_descriptor.set_height(height as u64);
        texture_descriptor.set_pixel_format(MTLPixelFormat::RGBA8Unorm);
        texture_descriptor.set_texture_type(MTLTextureType::D2);
        texture_descriptor.set_storage_mode(MTLStorageMode::Managed);
        texture_descriptor.set_usage(MTLTextureUsage::ShaderRead);

        let texture = device.new_texture(&texture_descriptor);

        let region = MTLRegion {
            origin: MTLOrigin { x: 0, y: 0, z: 0 },
            size: MTLSize {
                width: width as u64,
                height: height as u64,
                depth: 1,
            },
        };

        texture.replace_region(
            region,
            0,
            data.as_ptr() as *const std::ffi::c_void,
            (width * 4) as u64,
        );

        Ok(Image {
            source: source.to_string(),
            texture,
            width,
            height,
        })
    }
}

impl Clone for Image {
    fn clone(&self) -> Self {
        Image {
            source: self.source.clone(),
            texture: self.texture.clone(),
            width: self.width,
            height: self.height,
        }
    }
}
