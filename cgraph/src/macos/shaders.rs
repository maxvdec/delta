use metal::*;

use crate::{metal::MetalRenderer, object::Object};

const SHADERS: &'static str = "
#include <metal_stdlib>
using namespace metal;

struct VertexIn {
    float2 position [[attribute(0)]];
    float4 color [[attribute(1)]];
    float zIndex [[attribute(2)]];
    float2 uv [[attribute(3)]];
};

struct VertexOut {
    float4 position [[position]];
    float4 color;
    float2 uv;
};

struct Uniforms {
    float2 rect_position;
    float2 rect_size;
    float corner_radius;
    float4x4 model_matrix;
    float4x4 projection_matrix;
    uint use_texture;
    float shadow_radius;
    float4 shadow_color;
};

struct ShadowUniforms {
    float offset_x;
    float offset_y;
    float radius;
    float4 color;
    uint enabled;
};

vertex VertexOut vertex_main(VertexIn in [[stage_in]], constant Uniforms& uniforms [[buffer(1)]]) {
    VertexOut out;
    float depth = (0 + in.zIndex) / 50;
    out.position = uniforms.projection_matrix * uniforms.model_matrix * float4(in.position, depth, 1.0);
    out.color = in.color;
    out.uv = in.uv;
    return out;
}

float rounded_rect_sdf(float2 p, float2 size, float corner_radius) {
    float2 d = abs(p) - size + corner_radius;
    return length(max(d, 0.0)) + min(max(d.x, d.y), 0.0) - corner_radius;
}

// Improved shadow calculation with better falloff
float calculate_shadow(float2 uv, float2 rect_size, float corner_radius, float blur_radius) {
    // Convert UV to local coordinates centered at rectangle
    float2 local_pos = (uv - 0.5) * rect_size;
    float2 half_size = rect_size * 0.5;
    
    // Calculate distance from rounded rectangle edge
    float dist = rounded_rect_sdf(local_pos, half_size, corner_radius);
    
    // Create smooth falloff for professional shadow look
    // Using a more gradual falloff curve
    float shadow_alpha;
    if (dist < 0.0) {
        // Inside the shape - full shadow
        shadow_alpha = 1.0;
    } else {
        // Outside the shape - gradual falloff
        // Use exponential decay for more natural shadow
        float normalized_dist = dist / blur_radius;
        shadow_alpha = exp(-normalized_dist * 2.0); // Exponential falloff
        
        // Alternative: Gaussian-like falloff (uncomment to use)
        // shadow_alpha = exp(-normalized_dist * normalized_dist * 2.0);
    }
    
    return clamp(shadow_alpha, 0.0, 1.0);
}

fragment float4 fragment_main(VertexOut in [[stage_in]], 
                             constant Uniforms& uniforms [[buffer(0)]],
                             texture2d<float> tex [[texture(0)]],
                             sampler texSampler [[sampler(0)]],
                             constant ShadowUniforms& shadowUniforms [[buffer(2)]]) {
    
    // Shadow rendering pass
    if (shadowUniforms.enabled == 1) {
        float shadow_alpha = calculate_shadow(
            in.uv, 
            uniforms.rect_size, 
            uniforms.corner_radius, 
            shadowUniforms.radius
        );
        
        // Discard fragments with very low alpha to improve performance
        if (shadow_alpha <= 0.01) {
            discard_fragment();
        }
        
        float4 shadow_color = shadowUniforms.color;
        shadow_color.a *= shadow_alpha;
        return shadow_color;
    }
    
    // Main object rendering
    float4 final_color;
    
    if (uniforms.use_texture == 1) {
        float4 tex_color = tex.sample(texSampler, in.uv);
        final_color = tex_color;
    } else {
        final_color = in.color;
    }
    
    // Apply corner radius clipping for main object
    if (uniforms.corner_radius > 0.0) {
        float2 local_pos = (in.uv - 0.5) * uniforms.rect_size;
        float2 half_size = uniforms.rect_size * 0.5;
        
        float dist = rounded_rect_sdf(local_pos, half_size, uniforms.corner_radius);
        
        // Sharp edge for main object (not shadow)
        if (dist > 0.0) {
            discard_fragment();
        }
        
        // Optional: Add slight anti-aliasing to main object edge
        float alpha = 1.0 - smoothstep(-1.0, 1.0, dist);
        final_color.a *= alpha;
    }
    
    return final_color;
}";

pub fn create_library(device: &Device) -> Library {
    match device.new_library_with_source(SHADERS, &CompileOptions::new()) {
        Ok(library) => library,
        Err(e) => panic!("Failed to create shader library: {}", e),
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

impl MetalRenderer {
    pub fn render_object(
        &self,
        encoder: &RenderCommandEncoderRef,
        object: &Object,
        is_shadow: bool,
    ) {
        let buffer = &object.get_buffer().buffer;
        encoder.set_vertex_buffer(0, Some(&buffer), 0);

        let uniform_buffer = if is_shadow {
            object.make_shadow_position_uniforms(&self.layer)
        } else {
            object.make_uniforms(&self.layer)
        };

        encoder.set_vertex_buffer(1, Some(&uniform_buffer.buffer), 0);
        encoder.set_fragment_buffer(0, Some(&uniform_buffer.buffer), 0);

        let shadow_uniforms = if is_shadow {
            object.make_shadow_uniforms_enabled()
        } else {
            object.make_shadow_uniforms_disabled()
        };

        encoder.set_fragment_buffer(2, Some(&shadow_uniforms.buffer), 0);

        if object.use_texture && !is_shadow {
            if let Some(ref texture) = object.texture {
                encoder.set_fragment_texture(0, Some(&texture.texture));
                encoder.set_fragment_sampler_state(0, Some(&self.sampler));
            }
        }

        encoder.draw_indexed_primitives(
            MTLPrimitiveType::Triangle,
            object.indices.len() as u64,
            MTLIndexType::UInt32,
            &object.get_index_buffer().buffer,
            0,
        );
    }
}
