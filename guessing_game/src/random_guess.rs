use rand::Rng;

// Function to generate a random number based on user input
pub fn generate_random_number(user_input: i32) -> i32 {
    let mut rng = rand::thread_rng(); // Create a random number generator
    rng.gen_range(0..=user_input) // Generate a number in the range [0, user_input]
}

// Function to validate the user input
pub fn validate_input(input: i32) -> bool {
    input > 100 && input < 1000
}
