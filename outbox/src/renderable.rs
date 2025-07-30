#[derive(Debug, Clone)]
pub enum PaddingDirection {
    Top,
    Bottom,
    Left,
    Right,
    Vertical,
    Horizontal,
}

pub trait Renderable {
    fn render(
        &self,
        canvas_size: [f32; 2],
        assigned_position: [f32; 2],
    ) -> Vec<cgraph::object::Object>;
    fn get_size(&self) -> [f32; 2];
    fn get_padding(&self) -> [f32; 4];
    fn copy(&self) -> Box<dyn Renderable>;
}
