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
    fn padding(&mut self, padding: [f32; 4]) -> &mut dyn Renderable;
    fn padding_area(
        &mut self,
        direction: PaddingDirection,
        padding: [f32; 2],
    ) -> &mut dyn Renderable;
    fn padding_at(&mut self, direction: PaddingDirection, padding: f32) -> &mut dyn Renderable;
    fn get_padding(&self) -> [f32; 4];
    fn copy(&mut self) -> Box<dyn Renderable>;
}
