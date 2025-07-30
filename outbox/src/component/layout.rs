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

        for (index, element) in self.elements.iter().enumerate() {
            // Add spacing before each element except the first one
            if index > 0 {
                position[1] += self.spacing;
            }

            let element_objects = element.render(
                canvas_size,
                [
                    position[0] + element.get_padding()[0],
                    position[1] + element.get_padding()[1],
                ],
            );
            objects.extend(element_objects);

            let size = element.get_size();
            position[1] += size[1] + element.get_padding()[1] + element.get_padding()[3]; // Move down for the next element
        }
        objects
    }

    fn get_size(&self) -> [f32; 2] {
        let mut y_size = 0.0;
        let mut x_sizes = Vec::new();

        for (index, element) in self.elements.iter().enumerate() {
            let element_size = element.get_size();
            let element_padding = element.get_padding();

            x_sizes.push(element_size[0] + element_padding[0] + element_padding[2]); // width + left + right padding
            y_size += element_size[1] + element_padding[1] + element_padding[3]; // height + top + bottom padding

            // Add spacing between elements (not after the last one)
            if index > 0 {
                y_size += self.spacing;
            }
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
        self
    }

    /// Chainable version of add_spacing for use in builder pattern
    pub fn with_spacing(mut self, spacing: f32) -> Self {
        self.spacing = spacing;
        self
    }

    /// Add padding to column [left, top, right, bottom]
    pub fn padding(mut self, padding: [f32; 4]) -> Self {
        self.padding = padding;
        self
    }

    /// Add padding in a specific direction
    pub fn padding_at(
        mut self,
        direction: crate::renderable::PaddingDirection,
        padding: f32,
    ) -> Self {
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

    /// Add padding for vertical or horizontal areas
    pub fn padding_area(
        mut self,
        direction: crate::renderable::PaddingDirection,
        padding: [f32; 2],
    ) -> Self {
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

        for (index, element) in self.elements.iter().enumerate() {
            // Add spacing before each element except the first one
            if index > 0 {
                position[0] += self.spacing;
            }

            let element_objects = element.render(
                canvas_size,
                [
                    position[0] + element.get_padding()[0],
                    position[1] + element.get_padding()[1],
                ],
            );
            objects.extend(element_objects);

            let size = element.get_size();
            position[0] += size[0] + element.get_padding()[0] + element.get_padding()[2]; // Move right for the next element
        }
        objects
    }

    fn get_size(&self) -> [f32; 2] {
        let mut y_sizes = Vec::new();
        let mut x_size = 0.0;

        for (index, element) in self.elements.iter().enumerate() {
            let element_size = element.get_size();
            let element_padding = element.get_padding();

            y_sizes.push(element_size[1] + element_padding[1] + element_padding[3]); // height + top + bottom padding
            x_size += element_size[0] + element_padding[0] + element_padding[2]; // width + left + right padding

            // Add spacing between elements (not after the last one)
            if index > 0 {
                x_size += self.spacing;
            }
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

    pub fn add_spacing(mut self, spacing: f32) -> Self {
        self.spacing = spacing;
        self
    }

    /// Mutable version of add_spacing for when you have a mutable reference
    pub fn set_spacing(&mut self, spacing: f32) -> &mut Self {
        self.spacing = spacing;
        self
    }

    /// Add padding to row [left, top, right, bottom]
    pub fn padding(mut self, padding: [f32; 4]) -> Self {
        self.padding = padding;
        self
    }

    /// Add padding in a specific direction
    pub fn padding_at(
        mut self,
        direction: crate::renderable::PaddingDirection,
        padding: f32,
    ) -> Self {
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

    /// Add padding for vertical or horizontal areas
    pub fn padding_area(
        mut self,
        direction: crate::renderable::PaddingDirection,
        padding: [f32; 2],
    ) -> Self {
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
