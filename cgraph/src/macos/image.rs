use metal::*;
use std::path::Path;

#[derive(Debug)]
pub struct Image {
    pub source: String,
    pub texture: metal::Texture,
    pub width: u32,
    pub height: u32,
}

impl Image {
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
