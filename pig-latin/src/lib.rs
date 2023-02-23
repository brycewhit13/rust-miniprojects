// Function to convert word to pig latin
pub fn word_to_pig_latin(s: &str) -> String {
    // Convert string to lowercase
    let s = s.to_lowercase();
    // Convert string to vector of chars
    let mut chars: Vec<char> = s.chars().collect();
    // Get first and last char
    let first_char = chars[0];
    let last_char = chars[chars.len() - 1];
    // Check if first char is a vowel
    if first_char == 'a'
        || first_char == 'e'
        || first_char == 'i'
        || first_char == 'o'
        || first_char == 'u'
    {
        // Add "way" to end of string
        chars.push('w');
        chars.push('a');
        chars.push('y');

        // if punctuation, move it to the end
        if last_char == '.' || last_char == '!' || last_char == '?' {
            // Remove last char from vector
            chars.remove(chars.len() - 4);
            // Add last char to end of vector
            chars.push(last_char);
        }
    } else {
        // Remove first char from vector
        chars.remove(0);
        // Add first char to end of vector
        chars.push(first_char);
        // Add "ay" to end of vector
        chars.push('a');
        chars.push('y');

        // if punctuation, move it to the end
        if last_char == '.' || last_char == '!' || last_char == '?' {
            // Remove last char from vector
            chars.remove(chars.len() - 4);
            // Add last char to end of vector
            chars.push(last_char);
        }
    }
    // Convert vector of chars to string
    chars.into_iter().collect()
}

// Function to convert sentence to pig latin
pub fn sent_to_pig_latin(s: &str) {
    // Convert string to lowercase
    let s = s.to_lowercase();
    // Convert string to vector of words
    let words: Vec<&str> = s.split_whitespace().collect();
    // Loop through words
    for word in words {
        // Convert word to pig latin
        let pig_latin_word = word_to_pig_latin(word);
        // Print pig latin word
        print!("{} ", pig_latin_word);
    }
    // Print new line
    println!();
}
