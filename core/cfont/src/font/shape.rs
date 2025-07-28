use std::error::Error;

use crate::font::load::Font;
use font_kit::outline::OutlineSink;
use lyon::{
    path::Path,
    tessellation::{BuffersBuilder, FillOptions, FillTessellator, FillVertex, VertexBuffers},
};
use pathfinder_geometry::line_segment::LineSegment2F;
use pathfinder_geometry::vector::Vector2F;
use rustybuzz::{Face, UnicodeBuffer};

#[derive(Debug)]
pub struct TextVertex {
    pub position: [f32; 2],
}

#[derive(Debug)]
pub struct TextGeometry {
    pub vertices: Vec<TextVertex>,
    pub indices: Vec<u16>,
}

#[derive(Debug, Clone)]
pub struct TextTransform {
    pub font_size: f32,        // Desired font size in pixels
    pub position: [f32; 2],    // Position in pixels [x, y]
    pub canvas_size: [f32; 2], // Canvas size in pixels [width, height]
}

impl TextGeometry {
    pub fn bounding_box(&self) -> (f32, f32, f32, f32) {
        if self.vertices.is_empty() {
            return (0.0, 0.0, 0.0, 0.0);
        }

        let mut min_x = f32::INFINITY;
        let mut max_x = f32::NEG_INFINITY;
        let mut min_y = f32::INFINITY;
        let mut max_y = f32::NEG_INFINITY;

        for vertex in &self.vertices {
            let [x, y] = vertex.position;
            min_x = min_x.min(x);
            max_x = max_x.max(x);
            min_y = min_y.min(y);
            max_y = max_y.max(y);
        }

        (min_x, min_y, max_x, max_y)
    }

    pub fn transform_to_canvas(&mut self, transform: TextTransform, font_units_per_em: f32) {
        let (min_x, min_y, _, _) = self.bounding_box();

        let font_scale = transform.font_size / font_units_per_em;

        for vertex in &mut self.vertices {
            let [x, y] = vertex.position;

            let pixel_x = x * font_scale;
            let pixel_y = y * font_scale;

            let local_x = pixel_x - (min_x * font_scale);
            let local_y = pixel_y - (min_y * font_scale);

            let final_x = local_x + transform.position[0];
            let final_y = local_y + transform.position[1];

            vertex.position = [final_x, final_y];
        }
    }

    pub fn normalize_to_canvas(&mut self, canvas_size: [f32; 2]) {
        for vertex in &mut self.vertices {
            vertex.position[0] /= canvas_size[0];
            vertex.position[1] /= canvas_size[1];
        }
    }

    pub fn pixel_dimensions(&self) -> (f32, f32) {
        let (min_x, min_y, max_x, max_y) = self.bounding_box();
        (max_x - min_x, max_y - min_y)
    }
}

struct PathBuilderSink {
    builder: lyon::path::Builder,
}

impl PathBuilderSink {
    fn new() -> Self {
        Self {
            builder: Path::builder(),
        }
    }

    fn build(self) -> Path {
        self.builder.build()
    }
}

impl OutlineSink for PathBuilderSink {
    fn move_to(&mut self, to: Vector2F) {
        self.builder.begin(lyon::math::Point::new(to.x(), to.y()));
    }

    fn line_to(&mut self, to: Vector2F) {
        self.builder.line_to(lyon::math::Point::new(to.x(), to.y()));
    }

    fn quadratic_curve_to(&mut self, ctrl: Vector2F, to: Vector2F) {
        self.builder.quadratic_bezier_to(
            lyon::math::Point::new(ctrl.x(), ctrl.y()),
            lyon::math::Point::new(to.x(), to.y()),
        );
    }

    fn cubic_curve_to(&mut self, ctrl: LineSegment2F, to: Vector2F) {
        self.builder.cubic_bezier_to(
            lyon::math::Point::new(ctrl.from().x(), ctrl.from().y()),
            lyon::math::Point::new(ctrl.to().x(), ctrl.to().y()),
            lyon::math::Point::new(to.x(), to.y()),
        );
    }

    fn close(&mut self) {
        self.builder.end(true);
    }
}

pub fn produce_text(font: Font, text: &str) -> Result<TextGeometry, Box<dyn Error>> {
    let cfont_font = font.clone();
    let face: Result<Face<'_>, Box<dyn Error>> =
        Face::from_slice(&font.bytes, 0).ok_or_else(|| "Value was none".into());

    let font = cfont_font.core_font.unwrap();

    let mut buffer = UnicodeBuffer::new();
    buffer.push_str(text);
    let glyph_buffer = rustybuzz::shape(&face?, &[], buffer);

    let mut global_geometry: VertexBuffers<[f32; 2], u16> = VertexBuffers::new();
    let mut tess = FillTessellator::new();

    let mut cursor_x = 0.0f32;

    for glyph in glyph_buffer
        .glyph_infos()
        .iter()
        .zip(glyph_buffer.glyph_positions())
    {
        let (info, pos) = glyph;
        let gid = info.glyph_id;
        let x_offset = pos.x_offset as f32;
        let y_offset = pos.y_offset as f32;
        let x_advance = pos.x_advance as f32;

        let mut sink = PathBuilderSink::new();
        font.outline(gid, font_kit::hinting::HintingOptions::None, &mut sink)?;
        let path = sink.build();

        tess.tessellate_path(
            &path,
            &FillOptions::default(),
            &mut BuffersBuilder::new(&mut global_geometry, |v: FillVertex| {
                [
                    v.position().x + cursor_x + x_offset,
                    v.position().y + y_offset,
                ]
            }),
        )?;

        cursor_x += x_advance;
    }

    let vertices: Vec<TextVertex> = global_geometry
        .vertices
        .into_iter()
        .map(|pos| TextVertex { position: pos })
        .collect();

    Ok(TextGeometry {
        vertices,
        indices: global_geometry.indices,
    })
}
