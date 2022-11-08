use crate::rectangle::{Rectangle, Point};

mod rectangle;

fn main() {
    println!("Hello, world!");
    let top_left_point: Point = Point {
        x: 0.0,
        y: 15.0,
    };

    let bottom_right_point: Point = Point {
        x: 24.0,
        y: 0.0,
    };

    let rectangle = Rectangle { top_left: top_left_point, bottom_right: bottom_right_point };

    println!("Area of rectangle is {}", (&rectangle.area()))
}
