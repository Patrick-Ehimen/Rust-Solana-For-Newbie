/// Generates a password of a given length, using a set of characters chosen
/// by the caller.
///
/// # Arguments
///
/// * `length`: The length of the password to generate.
/// * `use_uppercase`: Whether to include uppercase letters in the password.
/// * `use_lowercase`: Whether to include lowercase letters in the password.
/// * `use_digits`: Whether to include digits in the password.
/// * `use_symbols`: Whether to include symbols in the password.
///
/// # Returns
///
/// A string of length `length`, containing a randomly generated password.
///
/// # Panics
///
/// If none of the character types are selected, this function will panic.
fn generate_password(
    length: usize,
    use_uppercase: bool,
    use_lowercase: bool,
    use_digits: bool,
    use_symbols: bool,
) -> String {
    use rand::seq::SliceRandom;
    use rand::Rng;

    let mut character_set: Vec<u8> = Vec::new();
    if use_uppercase {
        character_set.extend(b"ABCDEFGHIJKLMNOPQRSTUVWXYZ");
    }
    if use_lowercase {
        character_set.extend(b"abcdefghijklmnopqrstuvwxyz");
    }
    if use_digits {
        character_set.extend(b"0123456789");
    }
    if use_symbols {
        character_set.extend(b"!@#$%^&*()-_=+[]{}|;:,.<>/?");
    }

    if character_set.is_empty() {
        panic!("At least one character type must be selected.");
    }

    let mut rng = rand::thread_rng();
    (0..length)
        .map(|_| *character_set.choose(&mut rng).unwrap() as char)
        .collect()
}

/// Generates a password of a given length, using all character types.
///
/// # Arguments
///
/// * `length`: The length of the password to generate.
///
/// # Returns
///
/// A string of length `length`, containing a randomly generated password.
pub fn generate_random_password(length: usize) -> String {
    generate_password(length, true, true, true, true)
}
