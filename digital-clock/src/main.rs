// Import the clock module
mod clock;

// Import the chrono and env modules from the standard library
use chrono::Local;
use std::env;

fn main() {
    // Get the command line arguments
    let args: Vec<String> = env::args().collect();

    // Check if the user wants to show the date
    let show_date = args.contains(&String::from("--show-date"));

    // Clear the screen and hide the cursor
    print!("\x1b[2J");
    print!("\x1b[?25l");

    // Loop indefinitely
    loop {
        // Get the current time
        let t = Local::now();

        // Format the time as a string
        let time = t.format("%H:%M:%S").to_string();

        // If the user wants to show the date, format it as a string
        let date = if show_date {
            Some(t.format("%Y-%m-%d").to_string())
        } else {
            None
        };

        // Call the display_time function from the clock module
        clock::display_time(&time, date.as_deref());

        // Sleep for 999 milliseconds
        std::thread::sleep(std::time::Duration::from_millis(999));

        // Move the cursor up 8 lines
        print!("\x1b[8A");
    }
}
