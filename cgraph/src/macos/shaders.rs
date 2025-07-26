use metal::*;

const SHADERS: &'static str = "
#include <metal_stdlib>
using namespace metal;

struct VertexIn {
    float2 position [[attribute(0)]];
    float4 color [[attribute(1)]];
    float zIndex [[attribute(2)]];
};

struct VertexOut {
    float4 position [[position]];
    float4 color;
};

struct Uniforms {
    float2 rect_position;
    float2 rect_size;
    float corner_radius;
    float4x4 model_matrix;
};

vertex VertexOut vertex_main(VertexIn in [[stage_in]], constant Uniforms& uniforms [[buffer(1)]]) {
    VertexOut out;

    float depth = (0 + 50 - in.zIndex) / 50;
    out.position = uniforms.model_matrix * float4(in.position, depth, 1.0);
    out.color = in.color;
    return out;
}

fragment float4 fragment_main(VertexOut in [[stage_in]], constant Uniforms& uniforms [[buffer(0)]]) {
    return in.color;
}
";

pub fn create_library(device: &Device) -> Library {
    match device.new_library_with_source(SHADERS, &CompileOptions::new()) {
        Ok(library) => library,
        Err(e) => panic!("Failed to create shader library: {}", e),
    }
}
