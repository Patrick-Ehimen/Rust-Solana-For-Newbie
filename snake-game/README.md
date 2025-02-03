# Snake Game in Rust

This is a terminal-based Snake game implemented in Rust. The game allows players to control a snake, eat food, and grow in length while avoiding collisions with the walls and itself.

## Features

- Simple and intuitive controls
- Dynamic game state updates
- Terminal rendering of the game
- Score tracking

## Project Structure

```
snake-game
├── src
│   ├── main.rs       # Entry point of the application
│   ├── game.rs       # Game state and logic
│   ├── input.rs      # User input handling
│   └── render.rs     # Rendering the game to the terminal
├── Cargo.toml        # Rust project configuration
└── README.md         # Project documentation
```

## Getting Started

### Prerequisites

- Rust installed on your machine. You can install Rust by following the instructions at [rust-lang.org](https://www.rust-lang.org/tools/install).

### Running the Game

1. Clone the repository:
   ```
   git clone <repository-url>
   cd snake-game
   ```

2. Build the project:
   ```
   cargo build
   ```

3. Run the game:
   ```
   cargo run
   ```

### Controls

- Use the arrow keys to control the direction of the snake.
- Eat the food to grow and increase your score.
- Avoid running into the walls or the snake's own body.

## Contributing

Feel free to submit issues or pull requests to improve the game!