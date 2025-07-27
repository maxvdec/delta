use glam::{Mat4, Vec2, Vec4};
use memoffset::offset_of;
use metal::*;
use std::mem::size_of;
use winit::window::Window;

use crate::{
    macos::{
        shaders::{create_library, setup_alpha_blending},
        view::setup_layer,
    },
    object::{Object, Vertex},
    renderer::Renderer,
};

use core_graphics_types::geometry::CGSize;

pub struct MetalRenderer {
    device: Device,
    command_queue: CommandQueue,
    state: RenderPipelineState,
    depth_stencil_state: DepthStencilState,
    pub objects: Vec<crate::object::Object>,
    pub layer: MetalLayer,
    sampler: SamplerState,
    msaa_texture: Texture,
    depth_texture: Texture,
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
        pipeline_descriptor.set_sample_count(4);

        pipeline_descriptor.set_depth_attachment_pixel_format(MTLPixelFormat::Depth32Float);

        setup_alpha_blending(&pipeline_descriptor);

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

        set_vertex_descriptor(
            vertex_descriptor,
            offset_of!(Vertex, uv),
            3,
            MTLVertexFormat::Float2,
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

        // Create a sampler for texture sampling
        let sampler_descriptor = SamplerDescriptor::new();
        sampler_descriptor.set_min_filter(MTLSamplerMinMagFilter::Linear);
        sampler_descriptor.set_mag_filter(MTLSamplerMinMagFilter::Linear);
        sampler_descriptor.set_mip_filter(MTLSamplerMipFilter::Linear);
        sampler_descriptor.set_address_mode_s(MTLSamplerAddressMode::ClampToEdge);
        sampler_descriptor.set_address_mode_t(MTLSamplerAddressMode::ClampToEdge);
        let sampler = device.new_sampler(&sampler_descriptor);

        let msaa_texture_desc = TextureDescriptor::new();
        msaa_texture_desc.set_pixel_format(MTLPixelFormat::RGBA8Unorm);
        msaa_texture_desc.set_width(window.inner_size().width as u64);
        msaa_texture_desc.set_height(window.inner_size().height as u64);
        msaa_texture_desc.set_storage_mode(MTLStorageMode::Private);
        msaa_texture_desc.set_usage(MTLTextureUsage::RenderTarget);
        msaa_texture_desc.set_texture_type(MTLTextureType::D2Multisample);
        msaa_texture_desc.set_sample_count(4); // Enable MSAA with 4 samples
        let msaa_texture = device.new_texture(&msaa_texture_desc);

        let depth_texture = {
            let depth_desc = TextureDescriptor::new();
            depth_desc.set_pixel_format(MTLPixelFormat::Depth32Float);
            depth_desc.set_width(window.inner_size().width as u64);
            depth_desc.set_height(window.inner_size().height as u64);
            depth_desc.set_storage_mode(MTLStorageMode::Private);
            depth_desc.set_usage(MTLTextureUsage::RenderTarget);
            depth_desc.set_texture_type(MTLTextureType::D2Multisample);
            depth_desc.set_sample_count(4);
            device.new_texture(&depth_desc)
        };

        MetalRenderer {
            device,
            command_queue,
            state,
            depth_stencil_state,
            layer,
            objects: Vec::new(),
            sampler,
            msaa_texture,
            depth_texture,
        }
    }

    fn add_object(&mut self, object: crate::object::Object) {
        self.objects.push(object);
    }

    fn clear(&mut self) {
        self.objects.clear();
    }

    fn destroy(&self) {}

    fn render(&mut self, _window: &winit::window::Window) {
        let command_buffer = self.command_queue.new_command_buffer();
        let drawable = self.layer.next_drawable().expect("Failed to get drawable");

        self.depth_texture =
            self.create_depth_texture(drawable.texture().width(), drawable.texture().height());

        let render_pass_descriptor = RenderPassDescriptor::new();
        let color_attachment = render_pass_descriptor
            .color_attachments()
            .object_at(0)
            .unwrap();

        color_attachment.set_texture(Some(&self.msaa_texture));
        color_attachment.set_load_action(MTLLoadAction::Clear);
        color_attachment.set_resolve_texture(Some(&drawable.texture()));
        color_attachment.set_clear_color(MTLClearColor::new(0.0, 0.5, 1.0, 1.0)); // Blue background
        color_attachment.set_store_action(MTLStoreAction::MultisampleResolve);

        let depth_attachment = render_pass_descriptor.depth_attachment().unwrap();
        depth_attachment.set_texture(Some(&self.depth_texture));
        depth_attachment.set_load_action(MTLLoadAction::Clear);
        depth_attachment.set_clear_depth(1.0);
        depth_attachment.set_store_action(MTLStoreAction::DontCare);

        render_pass_descriptor.set_depth_attachment(Some(&depth_attachment));

        let encoder = command_buffer.new_render_command_encoder(&render_pass_descriptor);
        encoder.set_render_pipeline_state(&self.state);
        encoder.set_depth_stencil_state(&self.depth_stencil_state);
        encoder.set_cull_mode(MTLCullMode::None);

        // First pass: Render shadows for all objects that have shadows enabled
        for object in &self.objects {
            if object.shadow_on {
                let buffer = &object.get_buffer().buffer;
                encoder.set_vertex_buffer(0, Some(&buffer), 0);

                let shadow_uniform_buffer = object.make_shadow_position_uniforms(&self.layer);
                encoder.set_vertex_buffer(1, Some(&shadow_uniform_buffer.buffer), 0);
                encoder.set_fragment_buffer(0, Some(&shadow_uniform_buffer.buffer), 0);

                let shadow_uniforms = object.make_shadow_uniforms_enabled();
                encoder.set_fragment_buffer(2, Some(&shadow_uniforms.buffer), 0);

                encoder.draw_indexed_primitives(
                    MTLPrimitiveType::Triangle,
                    object.indices.len() as u64,
                    MTLIndexType::UInt32,
                    &object.get_index_buffer().buffer,
                    0,
                );
            }
        }

        // Second pass: Render main objects
        for object in &self.objects {
            let buffer = &object.get_buffer().buffer;
            encoder.set_vertex_buffer(0, Some(&buffer), 0);

            let uniform_buffer = object.make_uniforms(&self.layer);
            encoder.set_vertex_buffer(1, Some(&uniform_buffer.buffer), 0);
            encoder.set_fragment_buffer(0, Some(&uniform_buffer.buffer), 0);

            let shadow_uniforms = object.make_shadow_uniforms_disabled();
            encoder.set_fragment_buffer(2, Some(&shadow_uniforms.buffer), 0);

            if object.use_texture {
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

        encoder.end_encoding();

        command_buffer.present_drawable(&drawable);
        command_buffer.commit();
    }

    fn resize(&mut self, width: f64, height: f64) {
        self.layer.set_drawable_size(CGSize::new(width, height));
        self.update_msaa_texture(width as u64, height as u64);
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
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

#[allow(dead_code)]
#[derive(Debug)]
#[repr(C)]
pub struct Uniforms {
    pub rect_position: Vec2,
    pub rect_size: Vec2,
    pub corner_radius: f32,
    pub model_matrix: Mat4,
    pub projection_matrix: Mat4,
    pub use_texture: u32, // Metal doesn't have native bool in uniforms, use u32
    pub shadow_radius: f32,
    pub shadow_color: Vec4,
}

impl Object {
    fn make_uniforms(&self, layer: &MetalLayer) -> crate::object::buffer::Buffer<Uniforms> {
        let translation =
            Mat4::from_translation(Vec2::new(self.position.x, self.position.y).extend(0.0));
        let scale = Mat4::from_scale(Vec2::new(self.scale.x, self.scale.y).extend(1.0));

        let rotation = Mat4::from_rotation_z(self.rotation);

        let width = layer.drawable_size().width as f32;
        let height = layer.drawable_size().height as f32;
        let left = 0.0;
        let right = width;
        let top = 0.0;
        let bottom = height;
        let near = -100.0;
        let far = 100.0;

        let projection = Mat4::orthographic_rh(left, right, bottom, top, near, far);

        let rect_size = Vec2::new(
            self.original_pixel_size.x * self.scale.x,
            self.original_pixel_size.y * self.scale.y,
        );

        let model_matrix = translation * rotation * scale;
        let uniforms = Uniforms {
            rect_position: Vec2::new(self.position.x, self.position.y),
            rect_size,
            corner_radius: self.corner_radius,
            model_matrix,
            projection_matrix: projection,
            use_texture: if self.use_texture { 1 } else { 0 },
            shadow_radius: self.shadow_radius,
            shadow_color: self.shadow_color,
        };

        return crate::object::buffer::Buffer::new(vec![uniforms]);
    }
}

impl MetalRenderer {
    fn create_depth_texture(&self, width: u64, height: u64) -> Texture {
        let depth_desc = TextureDescriptor::new();
        depth_desc.set_pixel_format(MTLPixelFormat::Depth32Float);
        depth_desc.set_width(width);
        depth_desc.set_height(height);
        depth_desc.set_storage_mode(MTLStorageMode::Private);
        depth_desc.set_usage(MTLTextureUsage::RenderTarget);
        depth_desc.set_texture_type(MTLTextureType::D2Multisample);
        depth_desc.set_sample_count(4);
        self.device.new_texture(&depth_desc)
    }
    fn update_msaa_texture(&mut self, width: u64, height: u64) {
        let msaa_texture_desc = TextureDescriptor::new();
        msaa_texture_desc.set_pixel_format(MTLPixelFormat::RGBA8Unorm);
        msaa_texture_desc.set_width(width);
        msaa_texture_desc.set_height(height);
        msaa_texture_desc.set_storage_mode(MTLStorageMode::Private);
        msaa_texture_desc.set_usage(MTLTextureUsage::RenderTarget);
        msaa_texture_desc.set_texture_type(MTLTextureType::D2Multisample);
        msaa_texture_desc.set_sample_count(4);
        self.msaa_texture = self.device.new_texture(&msaa_texture_desc);
    }
}
