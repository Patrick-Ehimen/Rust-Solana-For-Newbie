#[derive(PartialEq, Clone)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn read_input() -> Option<Direction> {
    use std::io::{self, Read};
    let mut buffer = [0; 1];

    if io::stdin().read(&mut buffer).is_ok() {
        match buffer[0] {
            b'w' => Some(Direction::Up),
            b'a' => Some(Direction::Left),
            b's' => Some(Direction::Down),
            b'd' => Some(Direction::Right),
            _ => None,
        }
    } else {
        None
    }
}

fn update_direction(current: &Direction, input: Option<Direction>) -> Direction {
    if let Some(new_direction) = input {
        // Prevent the snake from reversing
        if (current == &Direction::Up && new_direction != Direction::Down)
            || (current == &Direction::Down && new_direction != Direction::Up)
            || (current == &Direction::Left && new_direction != Direction::Right)
            || (current == &Direction::Right && new_direction != Direction::Left)
        {
            return new_direction;
        }
    }
    current.clone()
}

pub fn handle_input(current_direction: &Direction) -> Direction {
    let input = read_input();
    update_direction(current_direction, input)
}
