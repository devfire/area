// Define a point in a 2D space
pub struct Point {
    pub(crate) x: f32,
    pub(crate) y: f32,
}

// A rectangle has two defining end points: top left & bottom right.
pub struct Rectangle { pub top_left: Point, pub bottom_right: Point }

impl Rectangle {
    //pub fn new(top_left: Point, bottom_right: Point) -> Self { Self { top_left, bottom_right } }

    pub fn area(&self) -> f32 {
        (self.bottom_right.x - self.top_left.x) * (self.top_left.y - self.bottom_right.y)
    }
}