// Define a point in a 2D space
pub struct Point {
    x: f32,
    y: f32,
}

impl Point {
    pub fn new(x: f32, y: f32) -> Self { Self { x, y } }
}

/*
A rectangle has two defining end points:
top left & bottom right.
*/
pub struct Rectangle { top_left: Point, bottom_right: Point }

impl Rectangle {
    pub fn new(top_left: Point, bottom_right: Point) -> Self { Self { top_left, bottom_right } }

    pub fn area(&self) -> f32 {
        (self.top_left.x - self.bottom_right.x) * (self.top_left.y - self.bottom_right.y)

        // unimplemented!()
    }
}