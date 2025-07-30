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
    fn padding(self: Box<Self>, padding: [f32; 4]) -> Box<dyn Renderable>;
    fn padding_area(
        self: Box<Self>,
        direction: PaddingDirection,
        padding: [f32; 2],
    ) -> Box<dyn Renderable>;
    fn padding_at(
        self: Box<Self>,
        direction: PaddingDirection,
        padding: f32,
    ) -> Box<dyn Renderable>;
    fn get_padding(&self) -> [f32; 4];
    fn copy(&self) -> Box<dyn Renderable>;
}
