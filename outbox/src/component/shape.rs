use cgraph::object::primitives::{Color, Position, Size, create_circle, create_rounded_quad};

use crate::{event::EventManager, renderable::Renderable};

pub enum Shape {
    Circle {
        radius: f32,
        color: Color,
        padding: [f32; 4],
        events: EventManager,
    },
    Rectangle {
        width: f32,
        height: f32,
        corner_radius: f32,
        color: Color,
        padding: [f32; 4],
        events: EventManager,
    },
}

impl Renderable for Shape {
    fn copy(&self) -> Box<dyn Renderable> {
        match self {
            Shape::Circle {
                radius,
                color,
                padding,
                events,
            } => Box::new(Shape::Circle {
                radius: *radius,
                color: *color,
                padding: *padding,
                events: events.clone(),
            }),
            Shape::Rectangle {
                width,
                height,
                corner_radius,
                color,
                padding,
                events,
            } => Box::new(Shape::Rectangle {
                width: *width,
                height: *height,
                corner_radius: *corner_radius,
                color: *color,
                padding: *padding,
                events: events.clone(),
            }),
        }
    }

    fn get_padding(&self) -> [f32; 4] {
        match self {
            Shape::Circle { padding, .. } => *padding,
            Shape::Rectangle { padding, .. } => *padding,
        }
    }

    fn get_size(&self) -> [f32; 2] {
        match self {
            Shape::Circle { radius, .. } => [*radius * 2.0, *radius * 2.0],
            Shape::Rectangle { width, height, .. } => [*width, *height],
        }
    }

    fn render(
        &self,
        _canvas_size: [f32; 2],
        assigned_position: [f32; 2],
    ) -> Vec<cgraph::object::Object> {
        match self {
            Shape::Circle { radius, color, .. } => {
                let position = [assigned_position[0], assigned_position[1]];
                vec![create_circle(
                    Size::new(*radius * 2.0, *radius * 2.0),
                    *color,
                    2.0,
                    Position::new(position[0], position[1]),
                )]
            }
            Shape::Rectangle {
                width,
                height,
                corner_radius,
                color,
                ..
            } => {
                let position = [assigned_position[0], assigned_position[1]];
                vec![create_rounded_quad(
                    Size::new(*width, *height),
                    *color,
                    2.0,
                    Position::new(position[0], position[1]),
                    *corner_radius,
                )]
            }
        }
    }

    fn get_event_handler(&self) -> Option<&EventManager> {
        match self {
            Shape::Circle { events, .. } => Some(events),
            Shape::Rectangle { events, .. } => Some(events),
        }
    }
}

impl Shape {
    pub fn new_rectangle(width: f32, height: f32, color: Color) -> Self {
        Shape::Rectangle {
            width,
            height,
            corner_radius: 0.0,
            color,
            padding: [0.0, 0.0, 0.0, 0.0],
            events: EventManager::default(),
        }
    }

    pub fn new_rounded_rectangle(
        width: f32,
        height: f32,
        color: Color,
        corner_radius: f32,
    ) -> Self {
        Shape::Rectangle {
            width,
            height,
            corner_radius,
            color,
            padding: [0.0, 0.0, 0.0, 0.0],
            events: EventManager::default(),
        }
    }

    pub fn new_circle(radius: f32, color: Color) -> Self {
        Shape::Circle {
            radius,
            color,
            padding: [0.0, 0.0, 0.0, 0.0],
            events: EventManager::default(),
        }
    }

    pub fn set_color(mut self, color: Color) -> Self {
        match &mut self {
            Shape::Circle { color: c, .. } => *c = color,
            Shape::Rectangle { color: c, .. } => *c = color,
        }
        self
    }
}
