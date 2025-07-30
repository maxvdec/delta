use cfont::font::{
    load::{Font, get_system_font, get_system_font_with_style},
    shape::{TextTransform, produce_styled_text},
    style::TextStyle,
};
use cgraph::{
    object::primitives::Color,
    text::{StyledFont, make_styled_text},
};
use glam::Vec2;

use crate::renderable::{PaddingDirection, Renderable};

#[derive(Clone)]
pub struct Text {
    pub content: String,
    pub font: Font,
    font_family: String,
    font_transform: cfont::font::shape::TextTransform,
    padding: [f32; 4],
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
            padding: [0.0, 0.0, 0.0, 0.0], // [left, top, right, bottom]
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
            padding: [0.0, 0.0, 0.0, 0.0], // [left, top, right, bottom]
            overrides_position: false,
            overrided_position: [0.0, 0.0],
        }
    }

    pub fn set_font(&mut self, font: Font) -> &mut Self {
        self.font = font;
        self
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

    pub fn override_position(&mut self, position: [f32; 2]) -> &mut Self {
        self.overrides_position = true;
        self.overrided_position = position;
        self
    }

    pub fn bold(&mut self) -> &mut Self {
        self.font_transform.style = self
            .font_transform
            .style
            .clone()
            .with_weight(cfont::font::style::FontWeight::Bold);
        let _ = self.reload_font_with_style();
        self
    }

    pub fn italic(&mut self) -> &mut Self {
        self.font_transform.style = self.font_transform.style.clone().with_italic(true);
        let _ = self.reload_font_with_style();
        self
    }

    pub fn underlined(&mut self) -> &mut Self {
        self.font_transform.style = self.font_transform.style.clone().with_underlined(true);
        let _ = self.reload_font_with_style();
        self
    }

    pub fn weight(&mut self, weight: cfont::font::style::FontWeight) -> &mut Self {
        self.font_transform.style = self.font_transform.style.clone().with_weight(weight);
        let _ = self.reload_font_with_style();
        self
    }

    pub fn thin(&mut self) -> &mut Self {
        self.font_transform.style = self
            .font_transform
            .style
            .clone()
            .with_weight(cfont::font::style::FontWeight::Thin);

        let _ = self.reload_font_with_style();
        self
    }

    pub fn light(&mut self) -> &mut Self {
        self.font_transform.style = self
            .font_transform
            .style
            .clone()
            .with_weight(cfont::font::style::FontWeight::Light);
        let _ = self.reload_font_with_style();
        self
    }

    pub fn extra_bold(&mut self) -> &mut Self {
        self.font_transform.style = self
            .font_transform
            .style
            .clone()
            .with_weight(cfont::font::style::FontWeight::ExtraBold);
        let _ = self.reload_font_with_style();
        self
    }

    pub fn set_size(&mut self, size: f32) -> &mut Self {
        self.font_transform.font_size = size;
        self
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
            Vec2::new(assigned_position[0], assigned_position[1]),
        );

        if object.is_err() {
            eprintln!("Failed to render text: {}", object.err().unwrap());
            return vec![];
        }

        vec![object.unwrap()]
    }

    fn get_size(&self) -> [f32; 2] {
        match produce_styled_text(self.font.clone(), &self.content, &self.font_transform.style) {
            Ok(mut geometry) => {
                let font_units_per_em = 1000.0;
                geometry.transform_to_canvas(self.font_transform.clone(), font_units_per_em);

                let (width, height) = geometry.pixel_dimensions();
                [width / 2.0, height / 2.0]
            }
            Err(_) => {
                let font_size = self.font_transform.font_size;
                let width = font_size * self.content.len() as f32 * 0.6;
                let height = font_size;
                [width, height]
            }
        }
    }

    fn padding(&mut self, padding: [f32; 4]) -> &mut dyn Renderable {
        self.padding = padding;
        self
    }

    fn padding_area(
        &mut self,
        direction: crate::renderable::PaddingDirection,
        padding: [f32; 2],
    ) -> &mut dyn Renderable {
        match direction {
            PaddingDirection::Vertical => {
                self.padding[1] = padding[0]; // Top
                self.padding[3] = padding[1]; // Bottom
            }
            PaddingDirection::Horizontal => {
                self.padding[0] = padding[0]; // Left
                self.padding[2] = padding[1]; // Right
            }
            _ => {
                panic!("Unsupported padding direction for Text component: {direction:?}");
            }
        }
        self
    }

    fn padding_at(&mut self, direction: PaddingDirection, padding: f32) -> &mut dyn Renderable {
        match direction {
            PaddingDirection::Top => self.padding[1] = padding,
            PaddingDirection::Bottom => self.padding[3] = padding,
            PaddingDirection::Left => self.padding[0] = padding,
            PaddingDirection::Right => self.padding[2] = padding,
            PaddingDirection::Vertical => {
                self.padding[1] = padding; // Top
                self.padding[3] = padding; // Bottom
            }
            PaddingDirection::Horizontal => {
                self.padding[0] = padding; // Left
                self.padding[2] = padding; // Right
            }
        }
        self
    }

    fn get_padding(&self) -> [f32; 4] {
        self.padding
    }

    fn copy(&mut self) -> Box<dyn Renderable> {
        Box::new(self.clone())
    }
}
