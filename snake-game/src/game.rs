use crate::input::Direction;

pub struct Game {
    pub snake: Vec<(i32, i32)>,
    pub direction: Direction,
    pub food: (i32, i32),
    pub score: i32,
    pub width: i32,
    pub height: i32,
}

impl Game {
    pub fn new(width: i32, height: i32) -> Self {
        let snake = vec![(width / 2, height / 2)];
        let direction = Direction::Right;
        let food = (0, 0); // Placeholder for food position
        let score = 0;

        Game {
            snake,
            direction,
            food,
            score,
            width,
            height,
        }
    }

    pub fn update(&mut self) {
        let head = self.snake[0];
        let new_head = match self.direction {
            Direction::Up => (head.0, head.1 - 1),
            Direction::Down => (head.0, head.1 + 1),
            Direction::Left => (head.0 - 1, head.1),
            Direction::Right => (head.0 + 1, head.1),
        };

        // Check for collisions with walls
        if new_head.0 < 0 || new_head.0 >= self.width || new_head.1 < 0 || new_head.1 >= self.height
        {
            // Handle game over
            return;
        }

        // Check for collisions with itself
        if self.snake.contains(&new_head) {
            // Handle game over
            return;
        }

        self.snake.insert(0, new_head);

        // Check for food collision
        if new_head == self.food {
            self.score += 1;
            self.food = self.generate_food();
        } else {
            self.snake.pop();
        }
    }

    pub fn draw(&self) {
        // Rendering logic will go here
    }

    pub fn change_direction(&mut self, direction: Direction) {
        self.direction = direction;
    }

    fn generate_food(&self) -> (i32, i32) {
        // Logic to generate food at a random position
        (0, 0) // Placeholder
    }
}
