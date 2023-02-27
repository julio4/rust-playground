#![allow(dead_code)]

#[derive(Debug)]
enum Color {
    Red,
    Green,
    Blue
}

#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
    color: Option<Color>
}

impl Point {
    // 2
    fn new(x: f64, y: f64) -> Self {
        Point { x, y, color: None }
    }

    // 3
    fn dist(&self, to: &Self) -> f64 {
        ((to.x - self.x).powi(2) + (to.y - self.y).powi(2)).sqrt()
    }
}

fn main() {
    // 1
    let origin: Point = Point {
        x: 0.0,
        y: 0.0,
        color: Some(Color::Red)
    };
    println!("Origin: {origin:?}");

    let some_point: Point = Point::new(origin.x + 10.0, origin.y + 10.0);
    println!(": {some_point:?}");

    println!("Distance between origin and some_point: {}", origin.dist(&some_point));
}
