// coordin.rs
// compile with file1.rs and file2.rs

#[derive(Default)]
pub struct Polar {
    pub distance: f64,  // distance from origin
    pub angle: f64,     // direction from origin
}

#[derive(Default)]
pub struct Rect {
    pub x: f64,         // horizontal distance from origin
    pub y: f64,         // vertical distance from origin
}
