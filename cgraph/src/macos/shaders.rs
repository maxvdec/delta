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

vertex VertexOut vertex_main(VertexIn in [[stage_in]]) {
    VertexOut out;
    
    out.position = float4(in.position, in.zIndex, 1.0);
    out.color = in.color;
    return out;
}

fragment float4 fragment_main(VertexOut in [[stage_in]]) {
    return in.color;
}
";

pub fn create_library(device: &Device) -> Library {
    match device.new_library_with_source(SHADERS, &CompileOptions::new()) {
        Ok(library) => library,
        Err(e) => panic!("Failed to create shader library: {}", e),
    }
}
