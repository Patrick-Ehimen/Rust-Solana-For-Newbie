# Command-Line Calculator

Welcome to the Command-Line Calculator! This is a simple calculator application written in Rust that allows users to perform basic arithmetic operations through the command line.

## Features

- Basic Arithmetic Operations: Supports addition (+), subtraction (-), multiplication (\*), and division (/).
- User-friendly prompts for input.
- Error Handling:
  - Invalid input or format prompts the user to try again.
  - Division by zero is gracefully handled.
- Loop for Continuous Use: Users can repeatedly perform calculations without restarting the program.
- Type 'exit' to quit the application.

## Getting Started

### Prerequisites

- Rust programming language installed on your machine. You can install Rust by following the instructions at [rust-lang.org](https://www.rust-lang.org/tools/install).

### Running the Calculator

1. Clone this repository or download the source code.
2. Navigate to the project directory.
3. Open a terminal and run the following command to start the calculator:

   ```bash
   cargo run
   ```

4. Follow the prompts to enter your calculations in the format: number1 operator number2 (e.g., 233 + 2).
5. Type exit to quit the calculator.

````Welcome to the Command-Line Calculator!

Please enter your calculation in the format: number1 operator number2 (233 + 2)
Supported operators: +, -, *, /
Type 'exit' to quit the calculator.```
````
