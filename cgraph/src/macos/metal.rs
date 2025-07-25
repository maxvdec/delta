use glam::{Mat4, Vec2, Vec4};
use memoffset::offset_of;
use metal::*;
use std::mem::size_of;
use winit::window::Window;

use crate::{
    macos::{shaders::create_library, view::setup_layer},
    object::{Object, Vertex},
    renderer::Renderer,
};

use core_graphics_types::geometry::CGSize;

pub struct MetalRenderer {
    device: Device,
    command_queue: CommandQueue,
    state: RenderPipelineState,
    depth_stencil_state: DepthStencilState,
    objects: Vec<crate::object::Object>,
    layer: MetalLayer,
}

impl Renderer for MetalRenderer {
    fn new(window: &Window) -> Self
    where
        Self: Sized,
    {
        let device = Device::system_default().expect("No Metal device found");
        let command_queue = device.new_command_queue();

        let pipeline_descriptor = RenderPipelineDescriptor::new();
        let library = create_library(&device);
        let vertex = library
            .get_function("vertex_main", None)
            .expect("Failed to get vertex function");
        let fragment = library
            .get_function("fragment_main", None)
            .expect("Failed to get fragment function");
        pipeline_descriptor.set_vertex_function(Some(&vertex));
        pipeline_descriptor.set_fragment_function(Some(&fragment));
        pipeline_descriptor
            .color_attachments()
            .object_at(0)
            .unwrap()
            .set_pixel_format(MTLPixelFormat::RGBA8Unorm);

        pipeline_descriptor.set_depth_attachment_pixel_format(MTLPixelFormat::Depth32Float);

        // Enable depth testing
        let depth_stencil_descriptor = DepthStencilDescriptor::new();
        depth_stencil_descriptor.set_depth_compare_function(MTLCompareFunction::LessEqual);
        depth_stencil_descriptor.set_depth_write_enabled(true);
        let depth_stencil_state = device.new_depth_stencil_state(&depth_stencil_descriptor);

        let vertex_descriptor = VertexDescriptor::new();

        set_vertex_descriptor(
            vertex_descriptor,
            offset_of!(Vertex, position),
            0,
            MTLVertexFormat::Float2,
        );

        set_vertex_descriptor(
            vertex_descriptor,
            offset_of!(Vertex, color),
            1,
            MTLVertexFormat::Float4,
        );

        set_vertex_descriptor(
            vertex_descriptor,
            offset_of!(Vertex, z_index),
            2,
            MTLVertexFormat::Float,
        );

        vertex_descriptor
            .layouts()
            .object_at(0)
            .unwrap()
            .set_stride(size_of::<Vertex>() as u64);
        vertex_descriptor
            .layouts()
            .object_at(0)
            .unwrap()
            .set_step_function(MTLVertexStepFunction::PerVertex);
        vertex_descriptor
            .layouts()
            .object_at(0)
            .unwrap()
            .set_step_rate(1);

        pipeline_descriptor.set_vertex_descriptor(Some(&vertex_descriptor));

        let state = match device.new_render_pipeline_state(&pipeline_descriptor) {
            Ok(state) => state,
            Err(e) => panic!("Failed to create render pipeline state: {}", e),
        };

        let layer = setup_layer(device.as_ref(), window);

        MetalRenderer {
            device,
            command_queue,
            state,
            depth_stencil_state,
            layer,
            objects: Vec::new(),
        }
    }

    fn add_object(&mut self, object: crate::object::Object) {
        self.objects.push(object);
    }

    fn clear(&mut self) {
        self.objects.clear();
    }

    fn destroy(&self) {}

    fn render(&self) {
        let command_buffer = self.command_queue.new_command_buffer();
        let drawable = self.layer.next_drawable().expect("Failed to get drawable");

        let depth_texture = self.set_depth(drawable.texture().width(), drawable.texture().height());

        let render_pass_descriptor = RenderPassDescriptor::new();
        let color_attachment = render_pass_descriptor
            .color_attachments()
            .object_at(0)
            .unwrap();

        color_attachment.set_texture(Some(&drawable.texture()));
        color_attachment.set_load_action(MTLLoadAction::Clear);
        color_attachment.set_clear_color(MTLClearColor::new(0.0, 0.5, 1.0, 1.0)); // Blue background
        color_attachment.set_store_action(MTLStoreAction::Store);

        let depth_attachment = render_pass_descriptor.depth_attachment().unwrap();
        depth_attachment.set_texture(Some(&depth_texture));
        depth_attachment.set_load_action(MTLLoadAction::Clear);
        depth_attachment.set_clear_depth(1.0);
        depth_attachment.set_store_action(MTLStoreAction::DontCare);

        render_pass_descriptor.set_depth_attachment(Some(&depth_attachment));

        let encoder = command_buffer.new_render_command_encoder(&render_pass_descriptor);
        encoder.set_render_pipeline_state(&self.state);
        encoder.set_depth_stencil_state(&self.depth_stencil_state);
        encoder.set_cull_mode(MTLCullMode::None);

        for object in &self.objects {
            let buffer = &object.get_buffer().buffer;
            encoder.set_vertex_buffer(0, Some(&buffer), 0);
            encoder.draw_indexed_primitives(
                MTLPrimitiveType::Triangle,
                object.indices.len() as u64,
                MTLIndexType::UInt32,
                &object.get_index_buffer().buffer,
                0,
            );
        }

        encoder.end_encoding();

        command_buffer.present_drawable(&drawable);
        command_buffer.commit();
    }

    fn resize(&self, width: f64, height: f64) {
        self.layer.set_drawable_size(CGSize::new(width, height));
    }
}

fn set_vertex_descriptor(
    vertex_descriptor: &VertexDescriptorRef,
    offset: usize,
    index: usize,
    format: MTLVertexFormat,
) {
    vertex_descriptor
        .attributes()
        .object_at(index as u64)
        .unwrap()
        .set_offset(offset as u64);
    vertex_descriptor
        .attributes()
        .object_at(index as u64)
        .unwrap()
        .set_format(format);
    vertex_descriptor
        .attributes()
        .object_at(index as u64)
        .unwrap()
        .set_buffer_index(0);
}

struct Uniforms {
    rect_position: Vec2,
    rect_size: Vec2,
    corner_radius: f32,
    model_matrix: Mat4,
}

impl Object {}

impl MetalRenderer {
    fn set_depth(&self, width: u64, height: u64) -> Texture {
        let depth_desc = TextureDescriptor::new();
        depth_desc.set_pixel_format(MTLPixelFormat::Depth32Float);
        depth_desc.set_width(width);
        depth_desc.set_height(height);
        depth_desc.set_storage_mode(MTLStorageMode::Private);
        depth_desc.set_usage(MTLTextureUsage::RenderTarget);
        let depth_texture = self.device.new_texture(&depth_desc);
        depth_texture
    }
}
