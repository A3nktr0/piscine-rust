#[derive(Debug)]
pub struct Circle {
    pub center: Point,
    pub radius: f64,
}

impl Circle {
    pub fn diameter(&self) -> f64 {
        self.radius * 2.0
    }
    pub fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius.powi(2)
    }
    pub fn intersect(&self, circle_b: &Circle) -> bool {
        self.center.distance(&circle_b.center) < self.radius + circle_b.radius
    }

    pub fn new(x: f64, y: f64, radius: f64) -> Circle {
        Circle {
            center: Point { x, y },
            radius,
        }
    }
}

#[derive(Debug)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub fn distance(&self, point_b: &Point) -> f64 {
        ((self.x - point_b.x).powi(2) + (self.y - point_b.y).powi(2)).sqrt()
    }
}
