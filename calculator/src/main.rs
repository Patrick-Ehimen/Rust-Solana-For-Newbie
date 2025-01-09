use std::io;

fn main() {
    println!("Welcome to the Command-Line Calculator!");

    loop {
        println!(
            "\nPlease enter your calculation in the format: number1 operator number2 (233 + 2)"
        );
        println!("Supported operators: +, -, *, /");
        println!("Type 'exit' to quit the calculator.");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        // Remove leading and trailing whitespace
        let input = input.trim();

        // Check if user wants to exit the calculator
        if input.eq_ignore_ascii_case("exit") {
            println!("Exiting the calculator. Goodbye!");
            break;
        }

        // Split the input into tokens (strings separated by whitespace)
        let tokens: Vec<&str> = input.split_whitespace().collect();

        // Check if the user entered a valid calculation
        if tokens.len() != 3 {
            println!("Invalid input. Please try again.");
            continue;
        }

        // Parse the first number from the input
        let number1: f64 = match tokens[0].parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid first number: {}", tokens[0]);
                continue;
            }
        };

        // Get the operator from the input
        let operator = tokens[1];

        // Parse the second number from the input
        let number2: f64 = match tokens[2].parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid second number: {}", tokens[2]);
                continue;
            }
        };

        let result = match operator {
            "+" => {
                // Add the two numbers
                number1 + number2
            }
            "-" => {
                // Subtract the second number from the first
                number1 - number2
            }
            "*" => {
                // Multiply the two numbers
                number1 * number2
            }
            "/" => {
                if number2 == 0.0 {
                    // Division by zero is not allowed
                    println!("Error: Division by zero is not allowed.");
                    continue;
                } else {
                    // Divide the first number by the second
                    number1 / number2
                }
            }
            _ => {
                // The operator is not supported
                println!(
                    "Invalid operator: {}. Supported operators are +, -, *, /",
                    operator
                );
                continue;
            }
        };

        println!("Result: {} {} {} = {}", number1, operator, number2, result);
    }
}
