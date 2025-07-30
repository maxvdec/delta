use crate::renderable::Renderable;

#[derive(Default)]
pub struct Column {
    pub elements: Vec<Box<dyn Renderable>>,
    spacing: f32,
    padding: [f32; 2],
}

impl Renderable for Column {
    fn render(
        &self,
        canvas_size: [f32; 2],
        assigned_position: [f32; 2],
    ) -> Vec<cgraph::object::Object> {
        let mut objects = Vec::new();
        let mut position = assigned_position;
        for element in &self.elements {
            let element_objects = element.render(canvas_size, position);
            objects.extend(element_objects);
            let size = element.get_size();
            position[1] += size[1]; // Move down for the next element
        }
        objects
    }

    fn get_size(&self) -> [f32; 2] {
        let mut y_size = 0.0;
        let mut x_sizes = Vec::new();
        for element in &self.elements {
            let element_size = element.get_size();
            x_sizes.push(element_size[0]);
            y_size += element_size[1];
        }
        let max = x_sizes
            .iter()
            .cloned()
            .filter(|v| !v.is_nan()) // Ignore NaN
            .fold(None, |acc, x| Some(acc.map_or(x, |a: f32| a.max(x))));

        if max.is_none() {
            return [0.0, y_size];
        }

        [max.unwrap(), y_size]
    }

    fn padding(&mut self, padding: [f32; 2]) {
        self.padding = padding;
        self.apply_padding_and_spacing();
    }
}

impl Column {
    pub fn add_element(&mut self, element: Box<dyn Renderable>) {
        self.elements.push(element);
    }

    pub fn add_spacing(&mut self, spacing: f32) {
        self.spacing = spacing;
        self.apply_padding_and_spacing();
    }

    fn apply_padding_and_spacing(&mut self) {
        if self.elements.is_empty() {
            return;
        }
        self.elements[0].padding([self.padding[0], self.padding[1]]);
        let mut y_offset = self.spacing;
        for element in self.elements.iter_mut().skip(1) {
            element.padding([self.padding[0], self.padding[1] + y_offset]);
            y_offset += self.spacing;
        }
    }
}

#[macro_export]
macro_rules! stack {
    ( $( $child:expr ),* $(,)?) => {
        {
            let mut col = Column::default();
            $(
                col.add_element(Box::new($child));
            )*
            col
        }
    };
}
