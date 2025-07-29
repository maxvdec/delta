use cfont::font::{
    load::{Font, get_system_font},
    render_text,
    shape::TextTransform,
    style::TextStyle,
};
use glam::Vec2;

use crate::renderable::Renderable;

pub struct Text {
    pub content: String,
    pub font: Font,
    font_transform: cfont::font::shape::TextTransform,
    padding: [f32; 2],
}

impl Default for Text {
    fn default() -> Self {
        Text {
            content: String::new(),
            font: Font::default(),
            font_transform: TextTransform {
                canvas_size: [0.0, 0.0],
                font_size: 16.0,
                position: [0.0, 0.0],
                style: TextStyle::new(),
            },
            padding: [0.0, 0.0],
        }
    }
}

impl Text {
    pub fn new(content: &str) -> Self {
        let font = if cfg!(target_os = "macos") {
            get_system_font("SF Pro").unwrap_or_else(|_| {
                eprintln!("Failed to load SF Pro font, using default font.");
                Font::default()
            })
        } else {
            get_system_font("Arial").unwrap_or_else(|_| {
                eprintln!("Failed to load Arial font, using default font.");
                Font::default()
            })
        };
        Text {
            content: content.to_string(),
            font,
            font_transform: TextTransform {
                canvas_size: [0.0, 0.0],
                font_size: 16.0,
                position: [0.0, 0.0],
                style: TextStyle::new(),
            },
            padding: [0.0, 0.0],
        }
    }

    pub fn set_font(&mut self, font: Font) {
        self.font = font;
    }

    pub fn set_font_by_name(&mut self, name: &str) -> Result<(), Box<dyn std::error::Error>> {
        self.font = get_system_font(name)?;
        Ok(())
    }

    pub fn bold(&mut self) {
        self.font_transform.style = self
            .font_transform
            .style
            .clone()
            .with_weight(cfont::font::style::FontWeight::Bold);
    }

    pub fn italic(&mut self) {
        self.font_transform.style = self.font_transform.style.clone().with_italic(true);
    }

    pub fn underlined(&mut self) {
        self.font_transform.style = self.font_transform.style.clone().with_underlined(true);
    }

    pub fn weight(&mut self, weight: cfont::font::style::FontWeight) {
        self.font_transform.style = self.font_transform.style.clone().with_weight(weight);
    }

    pub fn thin(&mut self) {
        self.font_transform.style = self
            .font_transform
            .style
            .clone()
            .with_weight(cfont::font::style::FontWeight::Thin);
    }

    pub fn light(&mut self) {
        self.font_transform.style = self
            .font_transform
            .style
            .clone()
            .with_weight(cfont::font::style::FontWeight::Light);
    }

    pub fn extra_bold(&mut self) {
        self.font_transform.style = self
            .font_transform
            .style
            .clone()
            .with_weight(cfont::font::style::FontWeight::ExtraBold);
    }

    pub fn set_size(&mut self, size: f32) {
        self.font_transform.font_size = size;
    }
}

impl Renderable for Text {
    fn render(
        &self,
        canvas_size: [f32; 2],
        assigned_position: [f32; 2],
    ) -> Vec<cgraph::object::Object> {
        let mut transform = self.font_transform.clone();
        transform.canvas_size = canvas_size;
        transform.position = [0.0, 0.0];

        // Use cfont's render_text function which properly handles styles
        let font_family = if cfg!(target_os = "macos") {
            "SF Pro"
        } else {
            "Arial"
        };

        let text_geometry = match render_text(font_family, &self.content, transform) {
            Ok(geometry) => geometry,
            Err(e) => {
                eprintln!("Error creating text geometry: {e:?}");
                return Vec::new();
            }
        };

        // Convert cfont geometry to cgraph objects
        let mut indices: Vec<u32> = vec![];
        for index in text_geometry.indices {
            indices.push(index as u32);
        }

        let mut vertices: Vec<cgraph::object::Vertex> = vec![];
        let color = cgraph::object::primitives::Color::new(1.0, 1.0, 1.0, 1.0);

        for vertex in text_geometry.vertices {
            let vertex = cgraph::object::Vertex {
                position: Vec2::new(
                    vertex.position[0] + assigned_position[0] + self.padding[0],
                    vertex.position[1] + assigned_position[1] + self.padding[1],
                ),
                color,
                z_index: 1.0,
                uv: Vec2::new(0.0, 0.0),
            };
            vertices.push(vertex);
        }

        if vertices.is_empty() {
            return Vec::new();
        }

        let mut object = cgraph::object::Object::new(vertices, indices);
        object.position = Vec2::new(
            assigned_position[0] + self.padding[0],
            assigned_position[1] + self.padding[1],
        );
        object.scale = Vec2::new(1.0, 1.0);
        object.original_pixel_size = Vec2::new(0.0, 0.0);
        object.rotation = 0.0;
        object.corner_radius = 0.0;

        vec![object]
    }

    fn get_size(&self) -> [f32; 2] {
        let font_size = self.font_transform.font_size;
        let width = font_size * self.content.len() as f32 * 0.6; // Approximate width based on character count
        let height = font_size; // Height is approximately the font size
        [width, height]
    }

    fn padding(&mut self, padding: [f32; 2]) {
        self.padding = padding;
    }
}
