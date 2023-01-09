#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, PartialEq)]
pub struct Line {
    pub start: Point,
    pub end: Point,
}
