// file2.rs -- contains functions called in file1.rs

use std::io;
use std::io::prelude::*;

use coordin;

// convert rectangular to polar coordinates
pub fn rect_to_polar(xypos: &coordin::Rect) -> coordin::Polar{
    coordin::Polar {
        distance: f64::sqrt(xypos.x * xypos.x + xypos.y * xypos.y),
        angle: f64::atan2(xypos.y, xypos.x),
    }
}

// show polar coorinates, converting angle to degrees
pub fn show_polar(dapos: &coordin::Polar) {
    let mut stdout: io::Stdout = io::stdout();

    const RAD_TO_DEG: f64 = 57.29577951;

    write!(stdout, "distance = {}", dapos.distance);
    write!(stdout, ", angle = {}", dapos.angle * RAD_TO_DEG);
    write!(stdout, " degress\n");
}
