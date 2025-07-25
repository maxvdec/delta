use metal::*;
use winit::window::Window;

use crate::{
    macos::{shaders::create_library, view::setup_layer},
    renderer::Renderer,
};

use core_graphics_types::geometry::CGSize;

pub struct MetalRenderer {
    command_queue: CommandQueue,
    state: RenderPipelineState,
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
            .set_pixel_format(MTLPixelFormat::BGRA8Unorm);

        let vertex_descriptor = VertexDescriptor::new();
        set_vertex_descriptor(vertex_descriptor, 0, 0, MTLVertexFormat::Float2); // Position
        set_vertex_descriptor(vertex_descriptor, 8, 1, MTLVertexFormat::Float4); // Color
        set_vertex_descriptor(vertex_descriptor, 24, 2, MTLVertexFormat::Float); // Z-Index

        vertex_descriptor
            .layouts()
            .object_at(0)
            .unwrap()
            .set_stride(32);
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
            command_queue,
            state,
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

        let render_pass_descriptor = RenderPassDescriptor::new();
        let color_attachment = render_pass_descriptor
            .color_attachments()
            .object_at(0)
            .unwrap();

        color_attachment.set_texture(Some(&drawable.texture()));
        color_attachment.set_load_action(MTLLoadAction::Clear);
        color_attachment.set_clear_color(MTLClearColor::new(0.0, 0.5, 1.0, 1.0)); // Blue background
        color_attachment.set_store_action(MTLStoreAction::Store);

        let encoder = command_buffer.new_render_command_encoder(&render_pass_descriptor);
        encoder.set_render_pipeline_state(&self.state);

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
