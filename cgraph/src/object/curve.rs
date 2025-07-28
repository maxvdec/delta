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
