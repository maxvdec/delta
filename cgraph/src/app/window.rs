pub struct Context {
    pub name: String,
    pub version: String,
    pub description: String,
}

pub struct Window {
    pub title: String,
    pub width: u32,
    pub height: u32,
}

impl Window {
    pub fn new(title: &str, width: u32, height: u32) -> Self {
        Window {
            title: title.to_string(),
            width,
            height,
        }
    }
}
