// Imports
use chrono;
use std::thread::sleep;

// Function to get the local time
pub fn get_local_time() -> String {
    let local = chrono::Local::now();
    local.format("%H:%M:%S").to_string()
}

pub fn main_loop() {
    // Infinite loop until canceled
    loop {
        // Check if q was pressed to stop the program
        if let Ok(true) = crossterm::event::poll(std::time::Duration::from_millis(0)) {
            if let Ok(crossterm::event::Event::Key(key)) = crossterm::event::read() {
                if key.code == crossterm::event::KeyCode::Char('q') {
                    break;
                }
            }
        }

        // Get the local time
        let time = get_local_time();

        // delete previous line in terminal
        print!("{}[1A", 27 as char);

        // Print the time
        println!("{}", time);

        // Sleep for 1 second
        sleep(std::time::Duration::from_millis(999));
    }
}