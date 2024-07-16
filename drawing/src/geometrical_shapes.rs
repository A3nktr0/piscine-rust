extern crate raster;
use rand::{rngs::ThreadRng, Rng};
use raster::Color;
use std::f32::consts::PI;

pub trait Drawable {
    fn draw(&self, image: &mut raster::Image);
    fn color(&self) -> raster::Color;
}

pub trait Displayable {
    fn display(&mut self, x: i32, y: i32, color: raster::Color);
}

// MARK: Point
#[derive(Copy, Clone)]
pub struct Point {
    x: i32,
    y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn random(width: i32, height: i32) -> Self {
        let mut rng: ThreadRng = rand::thread_rng();
        Self {
            x: rng.gen_range(0..width),
            y: rng.gen_range(0..height),
        }
    }
}

impl Drawable for Point {
    fn draw(&self, image: &mut raster::Image) {
        image.display(self.x, self.y, self.color());
    }

    fn color(&self) -> raster::Color {
        raster::Color::rgb(255, 255, 255)
    }
}

// MARK: Line
pub struct Line {
    start: Point,
    end: Point,
    color: Color,
}

impl Line {
    pub fn new(start: Point, end: Point, color: Color) -> Self {
        Self { start, end, color }
    }

    pub fn random(width: i32, height: i32) -> Self {
        Self {
            start: Point::random(width, height),
            end: Point::random(width, height),
            color: raster::Color::rgb(255, 255, 255),
        }
    }
}

impl Drawable for Line {
    //Bresenham’s Line Generation Algorithm
    fn draw(&self, image: &mut raster::Image) {
        let mut x0: i32 = self.start.x;
        let mut y0: i32 = self.start.y;
        let x1: i32 = self.end.x;
        let y1: i32 = self.end.y;

        // Calcul diff between start and end
        let dx: i32 = (x1 - x0).abs();
        let dy: i32 = (y1 - y0).abs();

        // Calcul signs for direction
        let sx: i32 = if x0 < x1 { 1 } else { -1 };
        let sy: i32 = if y0 < y1 { 1 } else { -1 };

        // Calcul intial error
        let mut err = if dx > dy { dx } else { -dy } / 2;
        let mut err2;

        loop {
            image.display(x0, y0, self.color.clone());
            if x0 == x1 && y0 == y1 {
                break;
            }
            err2 = err;
            if err2 > -dx {
                err -= dy;
                x0 += sx;
            }
            if err2 < dy {
                err += dx;
                y0 += sy;
            }
        }
    }

    fn color(&self) -> raster::Color {
        raster::Color::rgb(255, 255, 255)
    }
}

// MARK: Triangle
pub struct Triangle {
    a: Point,
    b: Point,
    c: Point,
}

impl Triangle {
    pub fn new(a: &Point, b: &Point, c: &Point) -> Self {
        Self {
            a: *a,
            b: *b,
            c: *c,
        }
    }
}

impl Drawable for Triangle {
    fn draw(&self, image: &mut raster::Image) {
        let color = self.color();

        Line::new(self.a, self.b, color.clone()).draw(image);
        Line::new(self.b, self.c, color.clone()).draw(image);
        Line::new(self.c, self.a, color.clone()).draw(image);
    }

    fn color(&self) -> raster::Color {
        let mut rng: ThreadRng = rand::thread_rng();
        raster::Color::rgb(
            rng.gen_range(0..255),
            rng.gen_range(0..255),
            rng.gen_range(0..255),
        )
    }
}

// MARK: Rectangle
pub struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

impl Rectangle {
    pub fn new(top_left: &Point, bottom_right: &Point) -> Self {
        Self {
            top_left: *top_left,
            bottom_right: *bottom_right,
        }
    }
}

impl Drawable for Rectangle {
    fn draw(&self, image: &mut raster::Image) {
        let top_right = Point::new(self.bottom_right.x, self.top_left.y);
        let bottom_left = Point::new(self.top_left.x, self.bottom_right.y);
        let color = self.color();

        Line::new(self.top_left, top_right, color.clone()).draw(image);
        Line::new(top_right, self.bottom_right, color.clone()).draw(image);
        Line::new(self.bottom_right, bottom_left, color.clone()).draw(image);
        Line::new(bottom_left, self.top_left, color.clone()).draw(image);
    }

    fn color(&self) -> raster::Color {
        let mut rng: ThreadRng = rand::thread_rng();
        raster::Color::rgb(
            rng.gen_range(0..255),
            rng.gen_range(0..255),
            rng.gen_range(0..255),
        )
    }
}

// MARK: Circle
pub struct Circle {
    center: Point,
    radius: i32,
}

impl Circle {
    pub fn _new(center: Point, radius: i32) -> Self {
        Self { center, radius }
    }

    pub fn random(width: i32, height: i32) -> Self {
        let mut rng: ThreadRng = rand::thread_rng();
        Self {
            center: Point::random(width, height),
            radius: rng.gen_range(1..width),
        }
    }
}

impl Drawable for Circle {
    fn draw(&self, image: &mut raster::Image) {
        let mut x: i32 = self.radius;
        let mut y: i32 = 0;
        let mut err: i32 = 0;
        let color = self.color();

        // Bresenham's circle algorithm - Method of Horn
        while x >= y {
            let points = [
                (self.center.x + x, self.center.y + y),
                (self.center.x + y, self.center.y + x),
                (self.center.x - y, self.center.y + x),
                (self.center.x - x, self.center.y + y),
                (self.center.x - x, self.center.y - y),
                (self.center.x - y, self.center.y - x),
                (self.center.x + y, self.center.y - x),
                (self.center.x + x, self.center.y - y),
            ];

            for (px, py) in points.iter() {
                image.display(*px, *py, color.clone());
            }

            if err <= 0 {
                y += 1;
                err += 2 * y + 1;
            } else {
                x -= 1;
                err -= 2 * x + 1;
            }
        }
    }

    fn color(&self) -> raster::Color {
        let mut rng: ThreadRng = rand::thread_rng();
        raster::Color::rgb(
            rng.gen_range(0..255),
            rng.gen_range(0..255),
            rng.gen_range(0..255),
        )
    }
}

// MARK: Pentagon
pub struct Pentagon {
    center: Point,
    radius: i32,
}

impl Pentagon {
    pub fn new(center: Point, radius: i32) -> Self {
        Self { center, radius }
    }
}

impl Drawable for Pentagon {
    fn draw(&self, image: &mut raster::Image) {
        let step = 2.0 * PI / 5.0; // Divide the circle into 5 parts
        let mut points = Vec::new();
        let color = self.color();

        for i in 0..5 {
            let angle = step * (i as f32);
            let x = self.center.x as f32 + self.radius as f32 * angle.cos();
            let y = self.center.y as f32 + self.radius as f32 * angle.sin();
            points.push(Point::new(x as i32, y as i32));
        }

        // Draw lines between the points
        for i in 0..5 {
            let start = points[i];
            let end = points[(i + 1) % 5];
            Line::new(start, end, color.clone()).draw(image);
        }
    }

    fn color(&self) -> raster::Color {
        let mut rng: ThreadRng = rand::thread_rng();
        raster::Color::rgb(
            rng.gen_range(0..255),
            rng.gen_range(0..255),
            rng.gen_range(0..255),
        )
    }
}

// MARK: Cube
pub struct Cube {
    top_left: Point,
    bottom_right: Point,
    offset: i32,
}

impl Cube {
    pub fn new(top_left: Point, bottom_right: Point) -> Self {
        Self {
            top_left,
            bottom_right,
            offset: 50,
        }
    }
}

impl Drawable for Cube {
    fn draw(&self, image: &mut raster::Image) {
        let top_right = Point::new(self.bottom_right.x, self.top_left.y);
        let bottom_left = Point::new(self.top_left.x, self.bottom_right.y);
        let color = self.color();

        Line::new(self.top_left, top_right, color.clone()).draw(image);
        Line::new(top_right, self.bottom_right, color.clone()).draw(image);
        Line::new(self.bottom_right, bottom_left, color.clone()).draw(image);
        Line::new(bottom_left, self.top_left, color.clone()).draw(image);

        let offset_point = |point: Point| Point::new(point.x + self.offset, point.y - self.offset);

        Line::new(self.top_left, offset_point(self.top_left), color.clone()).draw(image);
        Line::new(top_right, offset_point(top_right), color.clone()).draw(image);
        Line::new(
            self.bottom_right,
            offset_point(self.bottom_right),
            color.clone(),
        )
        .draw(image);
        Line::new(bottom_left, offset_point(bottom_left), color.clone()).draw(image);

        Line::new(
            offset_point(self.top_left),
            offset_point(top_right),
            color.clone(),
        )
        .draw(image);
        Line::new(
            offset_point(top_right),
            offset_point(self.bottom_right),
            color.clone(),
        )
        .draw(image);
        Line::new(
            offset_point(self.bottom_right),
            offset_point(bottom_left),
            color.clone(),
        )
        .draw(image);
        Line::new(
            offset_point(bottom_left),
            offset_point(self.top_left),
            color.clone(),
        )
        .draw(image);
    }

    fn color(&self) -> raster::Color {
        let mut rng: ThreadRng = rand::thread_rng();
        raster::Color::rgb(
            rng.gen_range(0..255),
            rng.gen_range(0..255),
            rng.gen_range(0..255),
        )
    }
}
