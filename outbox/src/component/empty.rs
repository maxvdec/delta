use crate::renderable::Renderable;

#[derive(Default)]
pub struct Empty {}

impl Renderable for Empty {
    fn render(
        &self,
        _canvas_size: [f32; 2],
        _assigned_position: [f32; 2],
    ) -> Vec<cgraph::object::Object> {
        Vec::new()
    }

    fn get_size(&self) -> [f32; 2] {
        [0.0, 0.0]
    }

    fn get_padding(&self) -> [f32; 4] {
        [0.0, 0.0, 0.0, 0.0]
    }

    fn copy(&self) -> Box<dyn Renderable> {
        Box::new(Empty::default())
    }
}
