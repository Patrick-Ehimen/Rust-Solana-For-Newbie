use chrono::Local;

// The DIGITS array is a 2D array of strings. The first dimension is the rows of the digits,
// and the second dimension is the columns of the digits. The characters in the strings are
// the characters that make up the digits.
//
// The first row of the DIGITS array is the top row of each digit. The second row is the
// second row of each digit, and so on. The characters in the strings are the characters
// that make up the top row of each digit, the second row of each digit, and so on.
//
// The characters in the strings are the characters that make up the digits. The characters
// at the beginning of each string are the characters that make up the left side of the
// digit, the characters in the middle of each string are the characters that make up the
// middle of the digit, and the characters at the end of each string are the characters
// that make up the right side of the digit.
//
// The characters that make up the digits are the characters that make up the numbers
// 0-9, as well as the colon and the space character.
const DIGITS: [[&str; 11]; 7] = [
    [
        "┏━┓ ",
        "  ╻  ",
        " ┏━┓ ",
        " ┏━┓ ",
        " ╻ ╻ ",
        " ┏━┓ ",
        " ┏   ",
        " ┏━┓ ",
        " ┏━┓ ",
        " ┏━┓ ",
        "   ",
    ],
    [
        "┃ ┃ ",
        "  ┃  ",
        "   ┃ ",
        "   ┃ ",
        " ┃ ┃ ",
        " ┃   ",
        " ┃   ",
        "   ┃ ",
        " ┃ ┃ ",
        " ┃ ┃ ",
        " ╻ ",
    ],
    [
        "┃ ┃ ",
        "  ┃  ",
        "   ┃ ",
        "   ┃ ",
        " ┃ ┃ ",
        " ┃   ",
        " ┃   ",
        "   ┃ ",
        " ┃ ┃ ",
        " ┃ ┃ ",
        "   ",
    ],
    [
        "┃ ┃ ",
        "  ┃  ",
        " ┏━┛ ",
        " ┣━┫ ",
        " ┗━┫ ",
        " ┗━┓ ",
        " ┣━┓ ",
        "   ┃ ",
        " ┣━┫ ",
        " ┗━┫ ",
        "   ",
    ],
    [
        "┃ ┃ ",
        "  ┃  ",
        " ┃   ",
        "   ┃ ",
        "   ┃ ",
        "   ┃ ",
        " ┃ ┃ ",
        "   ┃ ",
        " ┃ ┃ ",
        "   ┃ ",
        "   ",
    ],
    [
        "┃ ┃ ",
        "  ┃  ",
        " ┃   ",
        "   ┃ ",
        "   ┃ ",
        "   ┃ ",
        " ┃ ┃ ",
        "   ┃ ",
        " ┃ ┃ ",
        "   ┃ ",
        " ╹ ",
    ],
    [
        "┗━┛ ",
        "  ╹  ",
        " ┗━━ ",
        " ┗━┛ ",
        "   ╹ ",
        " ┗━┛ ",
        " ┗━┛ ",
        "   ╹ ",
        " ┗━┛ ",
        " ┗━┛ ",
        "   ",
    ],
];

pub fn display_time(time: &str, date: Option<&str>) {
    // This loop goes through each row of the DIGITS array.
    for row in &DIGITS {
        // This loop goes through each character of the time string.
        for c in time.chars() {
            // This line gets the column index of the character in the DIGITS array.
            let col = match c {
                '0'..='9' => c as usize - '0' as usize,
                _ => 10,
            };
            // This line prints the character at the current row and column.
            print!("{} ", row[col]);
        }
        // This line prints a newline after each row.
        println!();
    }

    // If there is a date, print it.
    if let Some(date) = date {
        println!("{}", date);
    }
}
