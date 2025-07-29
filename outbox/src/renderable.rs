pub trait Renderable {
    fn render(
        &self,
        canvas_size: [f32; 2],
        assigned_position: [f32; 2],
    ) -> Vec<cgraph::object::Object>;
    fn get_size(&self) -> [f32; 2];
    fn padding(&mut self, padding: [f32; 2]);
}
