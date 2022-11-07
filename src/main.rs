
// Define a point in a 2D space
struct Point {
    x: f32,
    y: f32,
}

// A rectangle has two defining end points
struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

fn rect_area(rect: &Rectangle) -> f32 {
    rect.top_left.y * rect.bottom_right.x
}

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

    let rectangle = Rectangle {
        top_left: top_left_point,
        bottom_right: bottom_right_point,
    };

    println!("Area of rectangle is {}", rect_area(&rectangle))
}
