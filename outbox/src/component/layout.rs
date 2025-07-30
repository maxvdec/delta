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

    fn padding(self: Box<Self>, padding: [f32; 4]) -> Box<dyn Renderable> {
        let mut col = *self;
        col.padding = padding;
        col.apply_padding_and_spacing();
        Box::new(col)
    }

    fn padding_at(
        self: Box<Self>,
        direction: crate::renderable::PaddingDirection,
        padding: f32,
    ) -> Box<dyn Renderable> {
        let mut col = *self;
        match direction {
            crate::renderable::PaddingDirection::Top => {
                col.padding[1] += padding;
            }
            crate::renderable::PaddingDirection::Bottom => {
                col.padding[3] += padding;
            }
            crate::renderable::PaddingDirection::Left => {
                col.padding[0] += padding;
            }
            crate::renderable::PaddingDirection::Right => {
                col.padding[2] += padding;
            }
            crate::renderable::PaddingDirection::Vertical => {
                col.padding[1] += padding;
                col.padding[3] += padding;
            }
            crate::renderable::PaddingDirection::Horizontal => {
                col.padding[0] += padding;
                col.padding[2] += padding;
            }
        }
        Box::new(col)
    }

    fn padding_area(
        self: Box<Self>,
        direction: crate::renderable::PaddingDirection,
        padding: [f32; 2],
    ) -> Box<dyn Renderable> {
        let mut col = *self;
        match direction {
            crate::renderable::PaddingDirection::Vertical => {
                col.padding[1] = padding[0]; // Top
                col.padding[3] = padding[1]; // Bottom
            }
            crate::renderable::PaddingDirection::Horizontal => {
                col.padding[0] = padding[0]; // Left
                col.padding[2] = padding[1]; // Right
            }
            _ => {
                panic!("Unsupported padding direction for Column component: {direction:?}");
            }
        }
        Box::new(col)
    }

    fn get_padding(&self) -> [f32; 4] {
        self.padding
    }

    fn copy(&self) -> Box<dyn Renderable> {
        let cloned_elements = self.elements.iter().map(|e| e.copy()).collect();
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

    fn apply_padding_and_spacing(&mut self) {
        if self.elements.is_empty() {
            return;
        }

        let mut new_elements = Vec::new();

        // Apply padding to first element
        if let Some(first) = self.elements.drain(..1).next() {
            new_elements.push(first.padding(self.padding));
        }

        let mut y_offset = self.spacing;
        for element in self.elements.drain(..) {
            let padded_element = element.padding([
                self.padding[0],
                self.padding[1] + y_offset,
                self.padding[2],
                self.padding[3],
            ]);
            new_elements.push(padded_element);
            y_offset += self.spacing;
        }

        self.elements = new_elements;
    }
}

#[macro_export]
macro_rules! stack {
    ( $( $child:expr ),* $(,)?) => {
        {
            let mut col = Column::default();
            $(
                col.elements.push(Box::new($child));
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

    fn padding(self: Box<Self>, padding: [f32; 4]) -> Box<dyn Renderable> {
        let mut row = *self;
        row.padding = padding;
        row.apply_padding_and_spacing();
        Box::new(row)
    }

    fn padding_at(
        self: Box<Self>,
        direction: crate::renderable::PaddingDirection,
        padding: f32,
    ) -> Box<dyn Renderable> {
        let mut row = *self;
        match direction {
            crate::renderable::PaddingDirection::Top => {
                row.padding[1] += padding;
            }
            crate::renderable::PaddingDirection::Bottom => {
                row.padding[3] += padding;
            }
            crate::renderable::PaddingDirection::Left => {
                row.padding[0] += padding;
            }
            crate::renderable::PaddingDirection::Right => {
                row.padding[2] += padding;
            }
            crate::renderable::PaddingDirection::Vertical => {
                row.padding[1] += padding;
                row.padding[3] += padding;
            }
            crate::renderable::PaddingDirection::Horizontal => {
                row.padding[0] += padding;
                row.padding[2] += padding;
            }
        }
        Box::new(row)
    }

    fn padding_area(
        self: Box<Self>,
        direction: crate::renderable::PaddingDirection,
        padding: [f32; 2],
    ) -> Box<dyn Renderable> {
        let mut row = *self;
        match direction {
            crate::renderable::PaddingDirection::Vertical => {
                row.padding[1] = padding[0]; // Top
                row.padding[3] = padding[1]; // Bottom
            }
            crate::renderable::PaddingDirection::Horizontal => {
                row.padding[0] = padding[0]; // Left
                row.padding[2] = padding[1]; // Right
            }
            _ => {
                panic!("Unsupported padding direction for Row component: {direction:?}");
            }
        }
        Box::new(row)
    }

    fn get_padding(&self) -> [f32; 4] {
        self.padding
    }

    fn copy(&self) -> Box<dyn Renderable> {
        let cloned_elements = self.elements.iter().map(|e| e.copy()).collect();
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

    fn apply_padding_and_spacing(&mut self) {
        if self.elements.is_empty() {
            return;
        }

        let mut new_elements = Vec::new();

        // Apply padding to first element
        if let Some(first) = self.elements.drain(..1).next() {
            new_elements.push(first.padding(self.padding));
        }

        // Apply padding and spacing to remaining elements
        let mut x_offset = self.spacing;
        for element in self.elements.drain(..) {
            let padded_element = element.padding([
                self.padding[0] + x_offset,
                self.padding[1],
                self.padding[2],
                self.padding[3],
            ]);
            new_elements.push(padded_element);
            x_offset += self.spacing;
        }

        self.elements = new_elements;
    }
}

#[macro_export]
macro_rules! row {
    ( $( $child:expr ),* $(,)?) => {
        {
            let mut row = Row::default();
            $(
                row.elements.push(Box::new($child));
            )*
            row
        }
    };
}
