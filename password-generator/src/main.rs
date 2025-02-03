use rand::Rng;
use std::io;

/// Generate a random password of a given length, using a set of characters chosen by the user.
fn main() {
    println!("Welcome to the Random Password Generator!");

    let mut length = String::new();
    println!("Enter the desired password length:");
    io::stdin()
        .read_line(&mut length)
        .expect("Failed to read line");
    let length: usize = length.trim().parse().expect("Please enter a valid number");

    let mut include_uppercase = String::new();
    println!("Include uppercase letters? (y/n):");
    io::stdin()
        .read_line(&mut include_uppercase)
        .expect("Failed to read line");

    let mut include_lowercase = String::new();
    println!("Include lowercase letters? (y/n):");
    io::stdin()
        .read_line(&mut include_lowercase)
        .expect("Failed to read line");

    let mut include_digits = String::new();
    println!("Include digits? (y/n):");
    io::stdin()
        .read_line(&mut include_digits)
        .expect("Failed to read line");

    let mut include_symbols = String::new();
    println!("Include symbols? (y/n):");
    io::stdin()
        .read_line(&mut include_symbols)
        .expect("Failed to read line");

    let password = generate_password(
        length,
        include_uppercase.trim().eq("y"),
        include_lowercase.trim().eq("y"),
        include_digits.trim().eq("y"),
        include_symbols.trim().eq("y"),
    );

    println!("Generated Password: {}", password);
}

/// Generate a random password of a given length, using a set of characters chosen by the user.
fn generate_password(
    length: usize,
    include_uppercase: bool,
    include_lowercase: bool,
    include_digits: bool,
    include_symbols: bool,
) -> String {
    let mut charset = String::new();
    if include_uppercase {
        charset.push_str("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
    }
    if include_lowercase {
        charset.push_str("abcdefghijklmnopqrstuvwxyz");
    }
    if include_digits {
        charset.push_str("0123456789");
    }
    if include_symbols {
        charset.push_str("!@#$%^&*()-_=+[]{}|;:,.<>?/");
    }

    let mut rng = rand::thread_rng();
    let password: String = (0..length)
        .map(|_| {
            let idx = rng.gen_range(0..charset.len());
            charset.chars().nth(idx).unwrap()
        })
        .collect();

    password
}
