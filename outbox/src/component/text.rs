use cfont::font::{
    load::{Font, get_system_font, get_system_font_with_style},
    shape::TextTransform,
    style::TextStyle,
};
use cgraph::{
    object::primitives::Color,
    text::{StyledFont, make_styled_text},
};
use glam::Vec2;

use crate::renderable::Renderable;

pub struct Text {
    pub content: String,
    pub font: Font,
    font_family: String,
    font_transform: cfont::font::shape::TextTransform,
    padding: [f32; 2],
    overrides_position: bool,
    overrided_position: [f32; 2],
}

impl Default for Text {
    fn default() -> Self {
        Text {
            content: String::new(),
            font: Font::default(),
            font_family: if cfg!(target_os = "macos") {
                "SF Pro".to_string()
            } else {
                "Arial".to_string()
            },
            font_transform: TextTransform {
                canvas_size: [0.0, 0.0],
                font_size: 16.0,
                position: [0.0, 0.0],
                style: TextStyle::new(),
            },
            padding: [0.0, 0.0],
            overrides_position: false,
            overrided_position: [0.0, 0.0],
        }
    }
}

impl Text {
    pub fn new(content: &str) -> Self {
        let font_family = if cfg!(target_os = "macos") {
            "SF Pro"
        } else {
            "Arial"
        };

        let font = get_system_font(font_family).unwrap_or_else(|_| {
            eprintln!("Failed to load {font_family} font, using default font.");
            Font::default()
        });

        Text {
            content: content.to_string(),
            font,
            font_family: font_family.to_string(),
            font_transform: TextTransform {
                canvas_size: [0.0, 0.0],
                font_size: 16.0,
                position: [0.0, 0.0],
                style: TextStyle::new(),
            },
            padding: [0.0, 0.0],
            overrides_position: false,
            overrided_position: [0.0, 0.0],
        }
    }

    pub fn set_font(&mut self, font: Font) {
        self.font = font;
    }

    pub fn set_font_by_name(&mut self, name: &str) -> Result<(), Box<dyn std::error::Error>> {
        self.font_family = name.to_string();
        self.reload_font_with_style()?;
        Ok(())
    }

    /// Reloads the font with the current style settings
    fn reload_font_with_style(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.font = get_system_font_with_style(&self.font_family, &self.font_transform.style)?;
        Ok(())
    }

    pub fn override_position(&mut self, position: [f32; 2]) {
        self.overrides_position = true;
        self.overrided_position = position;
    }

    pub fn bold(&mut self) {
        self.font_transform.style = self
            .font_transform
            .style
            .clone()
            .with_weight(cfont::font::style::FontWeight::Bold);
        let _ = self.reload_font_with_style();
    }

    pub fn italic(&mut self) {
        self.font_transform.style = self.font_transform.style.clone().with_italic(true);
        let _ = self.reload_font_with_style();
    }

    pub fn underlined(&mut self) {
        self.font_transform.style = self.font_transform.style.clone().with_underlined(true);
        let _ = self.reload_font_with_style();
    }

    pub fn weight(&mut self, weight: cfont::font::style::FontWeight) {
        self.font_transform.style = self.font_transform.style.clone().with_weight(weight);
        let _ = self.reload_font_with_style();
    }

    pub fn thin(&mut self) {
        self.font_transform.style = self
            .font_transform
            .style
            .clone()
            .with_weight(cfont::font::style::FontWeight::Thin);
        let _ = self.reload_font_with_style();
    }

    pub fn light(&mut self) {
        self.font_transform.style = self
            .font_transform
            .style
            .clone()
            .with_weight(cfont::font::style::FontWeight::Light);
        let _ = self.reload_font_with_style();
    }

    pub fn extra_bold(&mut self) {
        self.font_transform.style = self
            .font_transform
            .style
            .clone()
            .with_weight(cfont::font::style::FontWeight::ExtraBold);
        let _ = self.reload_font_with_style();
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

        let styled_font = StyledFont {
            core_font: self.font.clone(),
            transform,
            font_family: self.font_family.clone(),
        };

        let object = make_styled_text(
            styled_font,
            &self.content,
            Color::new(1.0, 1.0, 1.0, 1.0),
            1.0,
            Vec2::new(
                assigned_position[0] + self.padding[0],
                assigned_position[1] + self.padding[1],
            ),
        );

        if object.is_err() {
            eprintln!("Failed to render text: {}", object.err().unwrap());
            return vec![];
        }

        vec![object.unwrap()]
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
