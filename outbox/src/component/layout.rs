use crate::renderable::Renderable;

#[derive(Default)]
pub struct Column {
    pub elements: Vec<Box<dyn Renderable>>,
    spacing: f32,
    padding: [f32; 4],
}

impl Renderable for Column {
    fn render(
        &self,
        canvas_size: [f32; 2],
        assigned_position: [f32; 2],
    ) -> Vec<cgraph::object::Object> {
        let mut objects = Vec::new();
        let mut position = assigned_position;
        let mut last_bottom = 0.0;
        for element in &self.elements {
            let element_objects = element.render(
                canvas_size,
                [
                    position[0] + element.get_padding()[0],
                    position[1] + element.get_padding()[1] + last_bottom,
                ],
            );
            objects.extend(element_objects);
            let size = element.get_size();
            position[1] += size[1]; // Move down for the next element
            last_bottom = element.get_padding()[3];
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

    fn padding(&mut self, padding: [f32; 4]) -> &mut dyn Renderable {
        self.padding = padding;
        self.apply_padding_and_spacing();
        self
    }

    fn padding_at(
        &mut self,
        direction: crate::renderable::PaddingDirection,
        padding: f32,
    ) -> &mut dyn Renderable {
        match direction {
            crate::renderable::PaddingDirection::Top => {
                self.padding[1] += padding;
            }
            crate::renderable::PaddingDirection::Bottom => {
                self.padding[3] += padding;
            }
            crate::renderable::PaddingDirection::Left => {
                self.padding[0] += padding;
            }
            crate::renderable::PaddingDirection::Right => {
                self.padding[2] += padding;
            }
            crate::renderable::PaddingDirection::Vertical => {
                self.padding[1] += padding;
                self.padding[3] += padding;
            }
            crate::renderable::PaddingDirection::Horizontal => {
                self.padding[0] += padding;
                self.padding[2] += padding;
            }
        }
        self
    }

    fn padding_area(
        &mut self,
        direction: crate::renderable::PaddingDirection,
        padding: [f32; 2],
    ) -> &mut dyn Renderable {
        match direction {
            crate::renderable::PaddingDirection::Vertical => {
                self.padding[1] = padding[0]; // Top
                self.padding[3] = padding[1]; // Bottom
            }
            crate::renderable::PaddingDirection::Horizontal => {
                self.padding[0] = padding[0]; // Left
                self.padding[2] = padding[1]; // Right
            }
            _ => {
                panic!("Unsupported padding direction for Column component: {direction:?}");
            }
        }
        self
    }

    fn get_padding(&self) -> [f32; 4] {
        self.padding
    }

    fn copy(&mut self) -> Box<dyn Renderable> {
        let cloned_elements = self.elements.iter_mut().map(|e| e.copy()).collect();
        Box::new(Column {
            elements: cloned_elements,
            spacing: self.spacing,
            padding: self.padding,
        })
    }
}

impl Column {
    pub fn add_element(&mut self, element: Box<dyn Renderable>) -> &mut dyn Renderable {
        self.elements.push(element);
        self
    }

    pub fn add_spacing(&mut self, spacing: f32) -> &mut dyn Renderable {
        self.spacing = spacing;
        self.apply_padding_and_spacing();
        self
    }

    fn apply_padding_and_spacing(&mut self) -> &mut dyn Renderable {
        if self.elements.is_empty() {
            return self;
        }
        self.elements[0].padding(self.padding);
        let mut y_offset = self.spacing;
        for element in self.elements.iter_mut().skip(1) {
            element.padding([
                self.padding[0],
                self.padding[1] + y_offset,
                self.padding[2],
                self.padding[3],
            ]);
            y_offset += self.spacing;
        }
        self
    }
}

#[macro_export]
macro_rules! stack {
    ( $( $child:expr ),* $(,)?) => {
        {
            let mut col = Column::default();
            $(
                // For expressions that result in owned values
                let owned_value = $child.copy();
                col.add_element(owned_value);
            )*
            col
        }
    };
}

#[derive(Default)]
pub struct Row {
    pub elements: Vec<Box<dyn Renderable>>,
    spacing: f32,
    padding: [f32; 4],
}

impl Renderable for Row {
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
            position[0] += size[0]; // Move right for the next element
        }
        objects
    }

    fn get_size(&self) -> [f32; 2] {
        let mut y_sizes = Vec::new();
        let mut x_size = 0.0;
        for element in &self.elements {
            let element_size = element.get_size();
            y_sizes.push(element_size[1]);
            x_size += element_size[0];
        }

        if self.elements.len() > 1 {
            x_size += self.spacing * (self.elements.len() - 1) as f32;
        }

        let max = y_sizes
            .iter()
            .cloned()
            .filter(|v| !v.is_nan()) // Ignore NaN
            .fold(None, |acc, x| Some(acc.map_or(x, |a: f32| a.max(x))));

        if max.is_none() {
            return [x_size, 0.0];
        }

        [x_size, max.unwrap()]
    }

    fn padding(&mut self, padding: [f32; 4]) -> &mut dyn Renderable {
        self.padding = padding;
        self.apply_padding_and_spacing();
        self
    }

    fn padding_at(
        &mut self,
        direction: crate::renderable::PaddingDirection,
        padding: f32,
    ) -> &mut dyn Renderable {
        match direction {
            crate::renderable::PaddingDirection::Top => {
                self.padding[1] += padding;
            }
            crate::renderable::PaddingDirection::Bottom => {
                self.padding[3] += padding;
            }
            crate::renderable::PaddingDirection::Left => {
                self.padding[0] += padding;
            }
            crate::renderable::PaddingDirection::Right => {
                self.padding[2] += padding;
            }
            crate::renderable::PaddingDirection::Vertical => {
                self.padding[1] += padding;
                self.padding[3] += padding;
            }
            crate::renderable::PaddingDirection::Horizontal => {
                self.padding[0] += padding;
                self.padding[2] += padding;
            }
        }
        self
    }

    fn padding_area(
        &mut self,
        direction: crate::renderable::PaddingDirection,
        padding: [f32; 2],
    ) -> &mut dyn Renderable {
        match direction {
            crate::renderable::PaddingDirection::Vertical => {
                self.padding[1] = padding[0]; // Top
                self.padding[3] = padding[1]; // Bottom
            }
            crate::renderable::PaddingDirection::Horizontal => {
                self.padding[0] = padding[0]; // Left
                self.padding[2] = padding[1]; // Right
            }
            _ => {
                panic!("Unsupported padding direction for Row component: {direction:?}");
            }
        }
        self
    }

    fn get_padding(&self) -> [f32; 4] {
        self.padding
    }

    fn copy(&mut self) -> Box<dyn Renderable> {
        let cloned_elements = self.elements.iter_mut().map(|e| e.copy()).collect();
        Box::new(Row {
            elements: cloned_elements,
            spacing: self.spacing,
            padding: self.padding,
        })
    }
}

impl Row {
    pub fn add_element(&mut self, element: Box<dyn Renderable>) -> &mut Self {
        self.elements.push(element);
        self
    }

    pub fn add_spacing(&mut self, spacing: f32) -> &mut Self {
        self.spacing = spacing;
        self.apply_padding_and_spacing();
        self
    }

    fn apply_padding_and_spacing(&mut self) -> &mut Self {
        if self.elements.is_empty() {
            return self;
        }
        self.elements[0].padding(self.padding);
        let mut x_offset = self.spacing;
        for element in self.elements.iter_mut().skip(1) {
            element.padding([
                self.padding[0] + x_offset,
                self.padding[1],
                self.padding[2],
                self.padding[3],
            ]);
            x_offset += self.spacing;
        }
        self
    }
}

#[macro_export]
macro_rules! row {
    ( $( $child:expr ),* $(,)?) => {
        {
            let mut row = Row::default();
            $(
                let owned_value = $child.copy();
                row.add_element(owned_value);
            )*
            row
        }
    };
}
