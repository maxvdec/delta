#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct TextStyle {
    pub weight: FontWeight,
    pub italic: bool,
    pub underlined: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum FontWeight {
    #[default]
    Normal = 400,
    Thin = 100,
    ExtraLight = 200,
    Light = 300,
    Medium = 500,
    SemiBold = 600,
    Bold = 700,
    ExtraBold = 800,
    Black = 900,
}

impl TextStyle {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn bold() -> Self {
        Self {
            weight: FontWeight::Bold,
            italic: false,
            underlined: false,
        }
    }

    pub fn italic() -> Self {
        Self {
            weight: FontWeight::Normal,
            italic: true,
            underlined: false,
        }
    }

    pub fn underlined() -> Self {
        Self {
            weight: FontWeight::Normal,
            italic: false,
            underlined: true,
        }
    }

    pub fn with_weight(mut self, weight: FontWeight) -> Self {
        self.weight = weight;
        self
    }

    pub fn with_italic(mut self, italic: bool) -> Self {
        self.italic = italic;
        self
    }

    pub fn with_underlined(mut self, underlined: bool) -> Self {
        self.underlined = underlined;
        self
    }

    pub fn is_bold(&self) -> bool {
        matches!(
            self.weight,
            FontWeight::Bold | FontWeight::ExtraBold | FontWeight::Black
        )
    }

    /// Get the numeric weight value for font selection
    pub fn weight_value(&self) -> u16 {
        self.weight as u16
    }
}

impl FontWeight {
    /// Convert from a numeric weight value
    pub fn from_value(value: u16) -> Self {
        match value {
            0..=150 => FontWeight::Thin,
            151..=250 => FontWeight::ExtraLight,
            251..=350 => FontWeight::Light,
            351..=450 => FontWeight::Normal,
            451..=550 => FontWeight::Medium,
            551..=650 => FontWeight::SemiBold,
            651..=750 => FontWeight::Bold,
            751..=850 => FontWeight::ExtraBold,
            _ => FontWeight::Black,
        }
    }
}
