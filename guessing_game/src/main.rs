use std::io;

mod random_guess;

// This is the main function where the program starts execution
fn main() {
    let player_name = "Rustacean";
    println!("Welcome to the guessing game, {}!", player_name);

    loop {
        // Ask the user for the upper limit for the random number
        println!(
            "Please enter a number greater than 100 and less than 1000 (or type 'exit' to quit):"
        );

        // Create a mutable string to store the user's input
        let mut input = String::new();

        // Read the user's input
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        // Trim the input and check for exit command
        let trimmed_input = input.trim();
        if trimmed_input.eq_ignore_ascii_case("exit") || trimmed_input.eq_ignore_ascii_case("quit")
        {
            println!("Thanks for playing! Goodbye!");
            break; // Exit the loop to end the game
        }

        // Try to parse the input
        let user_input: i32 = match trimmed_input.parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input! Please enter a valid number.");
                continue; // Continue to the next iteration of the loop
            }
        };

        // Validate the user input
        if !random_guess::validate_input(user_input) {
            println!("Input must be greater than 100 and less than 1000. Please try again.");
            continue; // Continue to the next iteration of the loop
        }

        // Generate the random number based on user input
        let random_number = random_guess::generate_random_number(user_input);

        // The guessing game logic goes here...
        println!("Guess a number! {}", player_name);

        loop {
            // Ask the user for their guess
            println!("Please input your guess (or type 'exit' to quit).");

            // Create a mutable string to store the user's guess
            let mut guess = String::new();

            // Read the user's input
            io::stdin()
                .read_line(&mut guess)
                .expect("Failed to read line");

            // Trim the input and check for exit command
            let trimmed_guess = guess.trim();
            if trimmed_guess.eq_ignore_ascii_case("exit")
                || trimmed_guess.eq_ignore_ascii_case("quit")
            {
                println!("Thanks for playing! Goodbye!");
                break; // Exit the loop to end the game
            }

            match trimmed_guess.parse::<i32>() {
                Ok(number) => {
                    // If parsing is successful, check if the guess is correct
                    if number == random_number {
                        println!(
                            "Congratulations! You guessed the correct number: {}",
                            number
                        );

                        // Ask if the user wants to continue or end the game
                        println!("Do you want to continue playing? (yes/no)");
                        let mut continue_input = String::new();
                        io::stdin()
                            .read_line(&mut continue_input)
                            .expect("Failed to read line");

                        let trimmed_continue = continue_input.trim();
                        if trimmed_continue.eq_ignore_ascii_case("yes") {
                            break; // Break the inner loop to continue with a new upper limit
                        } else {
                            println!("Thanks for playing! Goodbye!");
                            return; // Exit the program
                        }
                    } else {
                        println!("Wrong guess! Try again.");
                    }
                }
                Err(_) => {
                    // If parsing fails, prompt the user to enter a number
                    println!("Wrong Input! Please enter a valid number.");
                }
            }
        }
    }
}
