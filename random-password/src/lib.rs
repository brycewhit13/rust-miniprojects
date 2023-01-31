/* Generates a random password according to the length and random seed parameters
By default the password is 8 characters long */

// Import rand crate
use rand::{self, rngs::ThreadRng, Rng};

// Establish possible numbers, letters, and symbols in a password
const NUMBERS: &str = "0123456789";
const UPPERCASE: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const LOWERCASE: &str = "abcdefghijklmnopqrstuvwxyz";
const SYMBOLS: &str = "~!@#$%^&*";

pub fn generate_password(length: usize) -> String {
    // Create a comprehensive character set
    let char_set: String = format!("{}{}{}{}", NUMBERS, UPPERCASE, LOWERCASE, SYMBOLS);
    let char_vec: Vec<char> = char_set.chars().collect(); // Allow it to be indexed
                                                          // Create a set for capitals and symbols to pull from individually
    let upper_vec: Vec<char> = UPPERCASE.chars().collect();
    let symbol_vec: Vec<char> = SYMBOLS.chars().collect();
    let num_vec: Vec<char> = NUMBERS.chars().collect();

    // Establish a ThreadRNG object for continued number generation
    let mut rng = rand::thread_rng();

    /*  Create a password by mapping each character in the password
    to a random character in the comprehensive character set */
    // Ensure one capital, number, and symbol are selected
    let capital_letter: char = upper_vec[rng.gen_range(0..upper_vec.len())];
    let number: char = num_vec[rng.gen_range(0..num_vec.len())];
    let symbol: char = symbol_vec[rng.gen_range(0..symbol_vec.len())];

    // Get the remaining letters
    let remaining_pass: String = (0..length - 3)
        .map(|_| {
            let rand_idx = rng.gen_range(0..char_vec.len());
            char_vec[rand_idx]
        })
        .collect();

    // Combine the results and return the password
    let password: String = shuffle_password(capital_letter, symbol, number, remaining_pass, rng);

    // return the password
    password
}

fn shuffle_password(capital_letter: char, symbol: char, number: char, remaining_pass: String, mut rng: ThreadRng) -> String {
    // Randomize the order so there is some variation with where the symbols and capital letters appear
    let password: String = match rng.gen_range(0..6) {
        0 => format!("{}{}{}{}", remaining_pass, capital_letter, symbol, number),
        1 => format!("{}{}{}{}", remaining_pass, capital_letter, number, symbol),
        2 => format!("{}{}{}{}", remaining_pass, symbol, capital_letter, number),
        3 => format!("{}{}{}{}", remaining_pass, symbol, number, capital_letter),
        4 => format!("{}{}{}{}", remaining_pass, number, capital_letter, symbol),
        5 => format!("{}{}{}{}", remaining_pass, number, symbol, capital_letter),
        _ => format!("{}{}{}{}", capital_letter, number, symbol, remaining_pass),
    };
    // Return the shuffled password
    password
}
