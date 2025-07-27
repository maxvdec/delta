use metal::*;

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
    bool use_texture;
};

struct ShadowUniforms {
    float offset_x;
    float offset_y;
    float radius;
    float4 color;
    bool enabled;
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

float calculate_shadow_expanded(float2 uv, float2 original_rect_size, float corner_radius, float blur_radius) {
    float expansion = blur_radius;
    
    float2 expanded_size = original_rect_size + float2(expansion * 2.0, expansion * 2.0);
    
    float2 expanded_pos = (uv - 0.5) * expanded_size;
    
    float2 half_original_size = original_rect_size * 0.5;
    float dist = rounded_rect_sdf(expanded_pos, half_original_size, corner_radius);
    
    float shadow_alpha = 1.0 - smoothstep(-blur_radius, blur_radius, dist);
    
    return clamp(shadow_alpha, 0.0, 1.0);
}

fragment float4 fragment_main(VertexOut in [[stage_in]], 
                             constant Uniforms& uniforms [[buffer(0)]],
                             texture2d<float> tex [[texture(0)]],
                             sampler texSampler [[sampler(0)]],
                             constant ShadowUniforms& shadowUniforms [[buffer(2)]]) {
    
    if (shadowUniforms.enabled) {
        float shadow_alpha = calculate_shadow_expanded(
            in.uv, 
            uniforms.rect_size, 
            uniforms.corner_radius, 
            shadowUniforms.radius
        );
        
        if (shadow_alpha <= 0.01) {
            discard_fragment();
        }
        
        float4 shadow_color = shadowUniforms.color;
        shadow_color.a *= shadow_alpha;
        return shadow_color;
    }
    
    float4 final_color;
    
    if (uniforms.use_texture) {
        float4 tex_color = tex.sample(texSampler, in.uv);
        final_color = tex_color;
    } else {
        final_color = in.color;
    }
    
    if (uniforms.corner_radius > 0.0) {
        float2 local_pos = (in.uv - 0.5) * uniforms.rect_size;
        float2 half_size = uniforms.rect_size * 0.5;
        
        float dist = rounded_rect_sdf(local_pos, half_size, uniforms.corner_radius);
        
        float alpha = 1.0 - smoothstep(-1.0, 1.0, dist);
        if (alpha <= 0.0) {
            discard_fragment();
        }
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
