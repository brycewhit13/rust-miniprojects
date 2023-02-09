/* CLI that moves your mouse in a triangle */

use autopilot::geometry::Point;
use autopilot::mouse::move_to;
use std::thread;

pub fn traingle_movement() {
    // Determine where to go
    let start = Point::new(100.0, 700.0);
    let middle = Point::new(600.0, 200.0);
    let end = Point::new(1200.0, 700.0);

    // Execute the movements
    _move_to(start);
    _move_to(middle);
    _move_to(end);
    _move_to(start);
}

// Function to handle errors with the move_to function
fn _move_to(point: Point) {
    if let Err(e) = move_to(point) {
        println!("{:?}", e);
        return;
    }
    thread::sleep(std::time::Duration::from_secs(1));
}
