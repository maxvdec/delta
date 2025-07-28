use crate::object::primitives::{Color, Position};
use crate::object::{
    Object,
    curve::{Curve, Path, Point},
};

pub fn create_quadratic_bezier(
    start: Position,
    control: Position,
    end: Position,
    color: Color,
    z_index: f32,
    line_width: f32,
    steps: usize,
) -> Object {
    let curve = Curve::Quadratic {
        p0: Point {
            x: start.x,
            y: start.y,
        },
        p1: Point {
            x: control.x,
            y: control.y,
        },
        p2: Point { x: end.x, y: end.y },
    };

    let path = Path {
        curves: vec![curve],
    };
    path.to_object(steps, color, z_index, line_width)
}

pub fn create_cubic_bezier(
    start: Position,
    control1: Position,
    control2: Position,
    end: Position,
    color: Color,
    z_index: f32,
    line_width: f32,
    steps: usize,
) -> Object {
    let curve = Curve::Cubic {
        p0: Point {
            x: start.x,
            y: start.y,
        },
        p1: Point {
            x: control1.x,
            y: control1.y,
        },
        p2: Point {
            x: control2.x,
            y: control2.y,
        },
        p3: Point { x: end.x, y: end.y },
    };

    let path = Path {
        curves: vec![curve],
    };
    path.to_object(steps, color, z_index, line_width)
}

pub fn create_arc(
    center: Position,
    radius: f32,
    start_angle: f32,
    end_angle: f32,
    color: Color,
    z_index: f32,
    line_width: f32,
    steps: usize,
) -> Object {
    let curve = Curve::Arc {
        center: Point {
            x: center.x,
            y: center.y,
        },
        radius,
        start_angle,
        end_angle,
    };

    let path = Path {
        curves: vec![curve],
    };
    path.to_object(steps, color, z_index, line_width)
}

pub fn create_circle_arc(
    center: Position,
    radius: f32,
    color: Color,
    z_index: f32,
    line_width: f32,
    steps: usize,
) -> Object {
    create_arc(
        center,
        radius,
        0.0,
        2.0 * std::f32::consts::PI,
        color,
        z_index,
        line_width,
        steps,
    )
}

pub fn create_catmull_rom_spline(
    points: Vec<Position>,
    tension: f32,
    color: Color,
    z_index: f32,
    line_width: f32,
    steps: usize,
) -> Object {
    if points.len() < 4 {
        // Not enough points for Catmull-Rom, return empty object
        return Object::new(vec![], vec![]);
    }

    let curve_points: Vec<Point> = points
        .iter()
        .map(|pos| Point { x: pos.x, y: pos.y })
        .collect();

    let curve = Curve::CatmullRom {
        points: curve_points,
        tension,
    };

    let path = Path {
        curves: vec![curve],
    };
    path.to_object(steps, color, z_index, line_width)
}

pub fn create_smooth_path(
    points: Vec<Position>,
    color: Color,
    z_index: f32,
    line_width: f32,
    steps: usize,
) -> Object {
    if points.len() < 2 {
        return Object::new(vec![], vec![]);
    }

    let mut curves = Vec::new();

    for i in 0..points.len() - 1 {
        let start = Point {
            x: points[i].x,
            y: points[i].y,
        };
        let end = Point {
            x: points[i + 1].x,
            y: points[i + 1].y,
        };

        // Create a simple control point at the midpoint for smoothness
        let control = Point {
            x: (start.x + end.x) / 2.0,
            y: (start.y + end.y) / 2.0,
        };

        let curve = Curve::Quadratic {
            p0: start,
            p1: control,
            p2: end,
        };

        curves.push(curve);
    }

    let path = Path { curves };
    path.to_object(steps, color, z_index, line_width)
}

pub fn create_path_from_points(
    points: Vec<Position>,
    color: Color,
    z_index: f32,
    line_width: f32,
    steps: usize,
) -> Object {
    if points.len() < 2 {
        return Object::new(vec![], vec![]);
    }

    let mut curves = Vec::new();

    // Create linear segments between consecutive points using quadratic curves with control point at midpoint
    for i in 0..points.len() - 1 {
        let start = Point {
            x: points[i].x,
            y: points[i].y,
        };
        let end = Point {
            x: points[i + 1].x,
            y: points[i + 1].y,
        };

        // For linear segments, place control point exactly at midpoint
        let control = Point {
            x: (start.x + end.x) / 2.0,
            y: (start.y + end.y) / 2.0,
        };

        let curve = Curve::Quadratic {
            p0: start,
            p1: control,
            p2: end,
        };

        curves.push(curve);
    }

    let path = Path { curves };
    path.to_object(steps, color, z_index, line_width)
}

pub struct PathBuilder {
    curves: Vec<Curve>,
    current_point: Option<Point>,
}

impl PathBuilder {
    pub fn new() -> Self {
        PathBuilder {
            curves: Vec::new(),
            current_point: None,
        }
    }

    pub fn move_to(mut self, position: Position) -> Self {
        self.current_point = Some(Point {
            x: position.x,
            y: position.y,
        });
        self
    }

    pub fn line_to(mut self, position: Position) -> Self {
        if let Some(start) = self.current_point {
            let end = Point {
                x: position.x,
                y: position.y,
            };
            let control = Point {
                x: (start.x + end.x) / 2.0,
                y: (start.y + end.y) / 2.0,
            };

            let curve = Curve::Quadratic {
                p0: start,
                p1: control,
                p2: end,
            };

            self.curves.push(curve);
            self.current_point = Some(end);
        }
        self
    }

    pub fn quadratic_to(mut self, control: Position, end: Position) -> Self {
        if let Some(start) = self.current_point {
            let curve = Curve::Quadratic {
                p0: start,
                p1: Point {
                    x: control.x,
                    y: control.y,
                },
                p2: Point { x: end.x, y: end.y },
            };

            self.curves.push(curve);
            self.current_point = Some(Point { x: end.x, y: end.y });
        }
        self
    }

    pub fn cubic_to(mut self, control1: Position, control2: Position, end: Position) -> Self {
        if let Some(start) = self.current_point {
            let curve = Curve::Cubic {
                p0: start,
                p1: Point {
                    x: control1.x,
                    y: control1.y,
                },
                p2: Point {
                    x: control2.x,
                    y: control2.y,
                },
                p3: Point { x: end.x, y: end.y },
            };

            self.curves.push(curve);
            self.current_point = Some(Point { x: end.x, y: end.y });
        }
        self
    }

    pub fn arc_to(
        mut self,
        center: Position,
        radius: f32,
        start_angle: f32,
        end_angle: f32,
    ) -> Self {
        let curve = Curve::Arc {
            center: Point {
                x: center.x,
                y: center.y,
            },
            radius,
            start_angle,
            end_angle,
        };

        self.curves.push(curve);

        // Update current point to the end of the arc
        let end_x = center.x + radius * end_angle.cos();
        let end_y = center.y + radius * end_angle.sin();
        self.current_point = Some(Point { x: end_x, y: end_y });
        self
    }

    pub fn build(self, color: Color, z_index: f32, line_width: f32, steps: usize) -> Object {
        let path = Path {
            curves: self.curves,
        };
        path.to_object(steps, color, z_index, line_width)
    }

    pub fn build_path(self) -> Path {
        Path {
            curves: self.curves,
        }
    }
}

impl Default for PathBuilder {
    fn default() -> Self {
        Self::new()
    }
}

pub fn create_heart_shape(
    center: Position,
    size: f32,
    color: Color,
    z_index: f32,
    line_width: f32,
    steps: usize,
) -> Object {
    let scale = size / 100.0; // Normalize to size

    PathBuilder::new()
        .move_to(Position::new(center.x, center.y + 25.0 * scale))
        .cubic_to(
            Position::new(center.x, center.y + 5.0 * scale),
            Position::new(center.x - 25.0 * scale, center.y - 20.0 * scale),
            Position::new(center.x - 25.0 * scale, center.y - 35.0 * scale),
        )
        .cubic_to(
            Position::new(center.x - 25.0 * scale, center.y - 50.0 * scale),
            Position::new(center.x, center.y - 50.0 * scale),
            Position::new(center.x, center.y - 35.0 * scale),
        )
        .cubic_to(
            Position::new(center.x, center.y - 50.0 * scale),
            Position::new(center.x + 25.0 * scale, center.y - 50.0 * scale),
            Position::new(center.x + 25.0 * scale, center.y - 35.0 * scale),
        )
        .cubic_to(
            Position::new(center.x + 25.0 * scale, center.y - 20.0 * scale),
            Position::new(center.x, center.y + 5.0 * scale),
            Position::new(center.x, center.y + 25.0 * scale),
        )
        .build(color, z_index, line_width, steps)
}

pub fn create_star_shape(
    center: Position,
    outer_radius: f32,
    inner_radius: f32,
    points: usize,
    color: Color,
    z_index: f32,
    line_width: f32,
    steps: usize,
) -> Object {
    if points < 3 {
        return Object::new(vec![], vec![]);
    }

    let mut builder = PathBuilder::new();
    let angle_step = std::f32::consts::PI / points as f32;

    // Start at the first outer point
    let start_angle = -std::f32::consts::PI / 2.0; // Start at top
    let start_x = center.x + outer_radius * start_angle.cos();
    let start_y = center.y + outer_radius * start_angle.sin();
    builder = builder.move_to(Position::new(start_x, start_y));

    for i in 0..points * 2 {
        let angle = start_angle + (i as f32 * angle_step);
        let radius = if i % 2 == 0 {
            inner_radius
        } else {
            outer_radius
        };
        let x = center.x + radius * angle.cos();
        let y = center.y + radius * angle.sin();
        builder = builder.line_to(Position::new(x, y));
    }

    builder = builder.line_to(Position::new(start_x, start_y));

    builder.build(color, z_index, line_width, steps)
}
