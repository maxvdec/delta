use crate::object::{Object, Vertex};
use glam::{Vec2, Vec4};

#[derive(Clone, Debug, Copy)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

pub enum Curve {
    Quadratic {
        p0: Point,
        p1: Point,
        p2: Point,
    },
    Cubic {
        p0: Point,
        p1: Point,
        p2: Point,
        p3: Point,
    },
    Arc {
        center: Point,
        radius: f32,
        start_angle: f32,
        end_angle: f32,
    },
    CatmullRom {
        points: Vec<Point>,
        tension: f32,
    },
}

pub trait CurveEval {
    fn evaluate(&self, t: f32) -> Point;
    fn points(&self, steps: usize) -> Vec<Point> {
        (0..=steps)
            .map(|i| self.evaluate(i as f32 / steps as f32))
            .collect()
    }
}

impl CurveEval for Curve {
    fn evaluate(&self, t: f32) -> Point {
        match self {
            Curve::Quadratic { p0, p1, p2 } => {
                let u = 1.0 - t;
                let x = u * u * p0.x + 2.0 * u * t * p1.x + t * t * p2.x;
                let y = u * u * p0.y + 2.0 * u * t * p1.y + t * t * p2.y;
                Point { x, y }
            }
            Curve::Cubic { p0, p1, p2, p3 } => {
                let u = 1.0 - t;
                let x = u * u * u * p0.x
                    + 3.0 * u * u * t * p1.x
                    + 3.0 * u * t * t * p2.x
                    + t * t * t * p3.x;
                let y = u * u * u * p0.y
                    + 3.0 * u * u * t * p1.y
                    + 3.0 * u * t * t * p2.y
                    + t * t * t * p3.y;
                Point { x, y }
            }
            Curve::Arc {
                center,
                radius,
                start_angle,
                end_angle,
            } => {
                let angle = start_angle + (end_angle - start_angle) * t;
                Point {
                    x: center.x + radius * angle.cos(),
                    y: center.y + radius * angle.sin(),
                }
            }
            Curve::CatmullRom { points, tension } => {
                if points.len() < 4 {
                    return Point { x: 0.0, y: 0.0 };
                }

                let t_scaled = t * (points.len() - 3) as f32;
                let i = t_scaled.floor() as usize;
                let t = t_scaled - i as f32;

                let p0 = points[i];
                let p1 = points[i + 1];
                let p2 = points[i + 2];
                let p3 = points[i + 3];

                let t2 = t * t;
                let t3 = t2 * t;

                let m1x = tension * (p2.x - p0.x);
                let m1y = tension * (p2.y - p0.y);
                let m2x = tension * (p3.x - p1.x);
                let m2y = tension * (p3.y - p1.y);

                Point {
                    x: (2.0 * t3 - 3.0 * t2 + 1.0) * p1.x
                        + (t3 - 2.0 * t2 + t) * m1x
                        + (-2.0 * t3 + 3.0 * t2) * p2.x
                        + (t3 - t2) * m2x,
                    y: (2.0 * t3 - 3.0 * t2 + 1.0) * p1.y
                        + (t3 - 2.0 * t2 + t) * m1y
                        + (-2.0 * t3 + 3.0 * t2) * p2.y
                        + (t3 - t2) * m2y,
                }
            }
        }
    }
}

pub struct Path {
    pub curves: Vec<Curve>,
}

impl Path {
    pub fn points(&self, steps: usize) -> Vec<Point> {
        self.curves.iter().flat_map(|c| c.points(steps)).collect()
    }
}

impl Curve {
    pub fn to_vertices(
        &self,
        steps: usize,
        color: Vec4,
        z_index: f32,
        line_width: f32,
    ) -> Vec<Vertex> {
        let points = self.points(steps);
        self.points_to_vertices(points, color, z_index, line_width)
    }

    pub fn to_line_vertices(
        &self,
        steps: usize,
        color: Vec4,
        z_index: f32,
        line_width: f32,
    ) -> (Vec<Vertex>, Vec<u32>) {
        let points = self.points(steps);
        self.points_to_line_strip(points, color, z_index, line_width)
    }

    fn points_to_vertices(
        &self,
        points: Vec<Point>,
        color: Vec4,
        z_index: f32,
        _line_width: f32,
    ) -> Vec<Vertex> {
        let point_count = points.len();
        points
            .into_iter()
            .enumerate()
            .map(|(i, point)| {
                let t = if point_count > 1 {
                    i as f32 / (point_count - 1) as f32
                } else {
                    0.0
                };
                Vertex::new(point.x, point.y, z_index, color, Vec2::new(t, 0.0))
            })
            .collect()
    }

    fn points_to_line_strip(
        &self,
        points: Vec<Point>,
        color: Vec4,
        z_index: f32,
        line_width: f32,
    ) -> (Vec<Vertex>, Vec<u32>) {
        if points.len() < 2 {
            return (vec![], vec![]);
        }

        let mut vertices = Vec::new();
        let mut indices = Vec::new();
        let half_width = line_width / 2.0;

        for i in 0..points.len() {
            let current = points[i];

            let (perp_x, perp_y) = if i == 0 {
                let next = points[i + 1];
                let dx = next.x - current.x;
                let dy = next.y - current.y;
                let length = (dx * dx + dy * dy).sqrt();
                if length > 0.0 {
                    (-dy / length, dx / length)
                } else {
                    (0.0, 1.0)
                }
            } else if i == points.len() - 1 {
                let prev = points[i - 1];
                let dx = current.x - prev.x;
                let dy = current.y - prev.y;
                let length = (dx * dx + dy * dy).sqrt();
                if length > 0.0 {
                    (-dy / length, dx / length)
                } else {
                    (0.0, 1.0)
                }
            } else {
                let prev = points[i - 1];
                let next = points[i + 1];
                let dx1 = current.x - prev.x;
                let dy1 = current.y - prev.y;
                let dx2 = next.x - current.x;
                let dy2 = next.y - current.y;
                let len1 = (dx1 * dx1 + dy1 * dy1).sqrt();
                let len2 = (dx2 * dx2 + dy2 * dy2).sqrt();

                let perp1 = if len1 > 0.0 {
                    (-dy1 / len1, dx1 / len1)
                } else {
                    (0.0, 1.0)
                };
                let perp2 = if len2 > 0.0 {
                    (-dy2 / len2, dx2 / len2)
                } else {
                    (0.0, 1.0)
                };

                let avg_x = (perp1.0 + perp2.0) / 2.0;
                let avg_y = (perp1.1 + perp2.1) / 2.0;
                let avg_len = (avg_x * avg_x + avg_y * avg_y).sqrt();
                if avg_len > 0.0 {
                    (avg_x / avg_len, avg_y / avg_len)
                } else {
                    (0.0, 1.0)
                }
            };

            let t = if points.len() > 1 {
                i as f32 / (points.len() - 1) as f32
            } else {
                0.0
            };

            let vertex_top = Vertex::new(
                current.x + perp_x * half_width,
                current.y + perp_y * half_width,
                z_index,
                color,
                Vec2::new(t, 0.0),
            );

            let vertex_bottom = Vertex::new(
                current.x - perp_x * half_width,
                current.y - perp_y * half_width,
                z_index,
                color,
                Vec2::new(t, 1.0),
            );

            vertices.push(vertex_top);
            vertices.push(vertex_bottom);

            if i < points.len() - 1 {
                let base = (i * 2) as u32;
                indices.extend_from_slice(&[
                    base,
                    base + 1,
                    base + 2, // First triangle
                    base + 1,
                    base + 3,
                    base + 2, // Second triangle
                ]);
            }
        }

        (vertices, indices)
    }
}

impl Path {
    pub fn to_vertices(
        &self,
        steps: usize,
        color: Vec4,
        z_index: f32,
        line_width: f32,
    ) -> Vec<Vertex> {
        self.curves
            .iter()
            .flat_map(|curve| curve.to_vertices(steps, color, z_index, line_width))
            .collect()
    }

    pub fn to_line_vertices(
        &self,
        steps: usize,
        color: Vec4,
        z_index: f32,
        line_width: f32,
    ) -> (Vec<Vertex>, Vec<u32>) {
        let mut all_vertices = Vec::new();
        let mut all_indices = Vec::new();

        for curve in &self.curves {
            let (mut vertices, mut indices) =
                curve.to_line_vertices(steps, color, z_index, line_width);

            let vertex_offset = all_vertices.len() as u32;
            for index in &mut indices {
                *index += vertex_offset;
            }

            all_vertices.append(&mut vertices);
            all_indices.append(&mut indices);
        }

        (all_vertices, all_indices)
    }

    pub fn to_object(&self, steps: usize, color: Vec4, z_index: f32, line_width: f32) -> Object {
        let (vertices, indices) = self.to_line_vertices(steps, color, z_index, line_width);
        let mut object = Object::new(vertices, indices);
        object.position = Vec2::new(0.0, 0.0);
        object.scale = Vec2::new(1.0, 1.0);
        object.rotation = 0.0;
        object.corner_radius = 0.0;
        object.update_buffer();
        object
    }
}
