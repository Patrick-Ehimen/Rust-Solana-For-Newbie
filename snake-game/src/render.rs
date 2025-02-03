use crate::game::Game;

pub fn render(game_state: &Game) {
    // Clear the terminal
    print!("{}[2J", 27 as char);
    print!("{}[H", 27 as char);

    // Draw the game borders
    for _ in 0..game_state.width {
        print!("#");
    }
    println!();

    // Draw the game area
    for y in 0..game_state.height {
        for x in 0..game_state.width {
            if game_state.snake.contains(&(x, y)) {
                print!("O"); // Draw the snake
            } else if (x, y) == game_state.food {
                print!("*"); // Draw the food
            } else {
                print!(" "); // Empty space
            }
        }
        println!();
    }

    // Draw the game borders
    for _ in 0..game_state.width {
        print!("#");
    }
    println!();

    // Display the score
    println!("Score: {}", game_state.score);
}
