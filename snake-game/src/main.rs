use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use std::io::{self, Write};
use std::thread;
use std::time::Duration;

mod game;
mod input;
mod render;

use game::Game;
use input::handle_input;
use render::render;

fn main() {
    // Enable raw mode to capture user input
    enable_raw_mode().unwrap();

    let mut game = Game::new(20, 20); // Provide width and height for the game

    loop {
        // Handle user input
        let direction = handle_input(&game.direction);
        game.change_direction(direction);

        // Update game state
        game.update();

        // Render the game state
        render(&game);

        // Flush the output to ensure it is displayed
        io::stdout().flush().unwrap();

        // Sleep to control the game speed
        thread::sleep(Duration::from_millis(100));
    }

    // Disable raw mode before exiting
    disable_raw_mode().unwrap();
}
