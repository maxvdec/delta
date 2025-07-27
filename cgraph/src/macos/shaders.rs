use crate::macos::shader_code::SHADER_CODE;
use metal::*;

pub fn create_library(device: &Device) -> Library {
    match device.new_library_with_source(SHADER_CODE, &CompileOptions::new()) {
        Ok(library) => library,
        Err(e) => panic!("Failed to create shader library: {e}"),
    }
}

pub fn setup_alpha_blending(pipeline_descriptor: &RenderPipelineDescriptorRef) {
    let color_attachment = pipeline_descriptor
        .color_attachments()
        .object_at(0)
        .unwrap();

    color_attachment.set_blending_enabled(true);
    color_attachment.set_source_rgb_blend_factor(MTLBlendFactor::SourceAlpha);
    color_attachment.set_destination_rgb_blend_factor(MTLBlendFactor::OneMinusSourceAlpha);
    color_attachment.set_rgb_blend_operation(MTLBlendOperation::Add);
    color_attachment.set_source_alpha_blend_factor(MTLBlendFactor::One);
    color_attachment.set_destination_alpha_blend_factor(MTLBlendFactor::OneMinusSourceAlpha);
    color_attachment.set_alpha_blend_operation(MTLBlendOperation::Add);
}
