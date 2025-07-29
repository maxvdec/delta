#[derive(Debug, Clone, PartialEq, Eq, Default)]
/// Represents a text style with properties such as weight, italic, and underlined.
pub struct TextStyle {
    /// The font weight, represented as a `FontWeight`.
    pub weight: FontWeight,
    /// Whether the text is italic.
    pub italic: bool,
    /// Whether the text is underlined.
    pub underlined: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
/// Represents the weight of a font.
pub enum FontWeight {
    #[default]
    /// Normal weight (400).
    Normal = 400,
    /// Thin weight (100).
    Thin = 100,
    /// Extra light weight (200)
    ExtraLight = 200,
    /// Light weight (300).
    Light = 300,
    /// Medium weight (500).
    Medium = 500,
    /// Semi-bold weight (600).
    SemiBold = 600,
    /// Bold weight (700).
    Bold = 700,
    /// Extra bold weight (800).
    ExtraBold = 800,
    /// Black weight (900).
    Black = 900,
}

impl TextStyle {
    /// Creates a new `TextStyle` with default properties.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a `TextStyle` with bold weight.
    pub fn bold() -> Self {
        Self {
            weight: FontWeight::Bold,
            italic: false,
            underlined: false,
        }
    }

    /// Creates a `TextStyle` with italic style.
    pub fn italic() -> Self {
        Self {
            weight: FontWeight::Normal,
            italic: true,
            underlined: false,
        }
    }

    /// Creates a `TextStyle` with underlined style.
    pub fn underlined() -> Self {
        Self {
            weight: FontWeight::Normal,
            italic: false,
            underlined: true,
        }
    }

    /// Creates a `TextStyle` with the specified weight, italic, and underlined properties.
    pub fn with_weight(mut self, weight: FontWeight) -> Self {
        self.weight = weight;
        self
    }

    /// Sets the italic property of the text style.
    pub fn with_italic(mut self, italic: bool) -> Self {
        self.italic = italic;
        self
    }

    /// Sets the underlined property of the text style.
    pub fn with_underlined(mut self, underlined: bool) -> Self {
        self.underlined = underlined;
        self
    }

    /// Checks if the text style is bold.
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
