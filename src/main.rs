use crate::rectangle::Rectangle;

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

    let rectangle = Rectangle { field1: top_left_point, field2: bottom_right_point };

    println!("Area of rectangle is {}", rect_area(&rectangle))
}
