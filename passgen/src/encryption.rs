// Take a string and a key to perform a shift cipher encryption and
// return the result.
pub fn encrypt_pass(pass: String, key: String) -> String {
    // Init output string
    let mut out = String::with_capacity(pass.len());

    // Counter for which character of the key we are on
    let mut i = 0;

    // Turn the key into a character vector and save
    // the vector length
    let key_chars: Vec<char> = key.chars().collect();
    let key_len = key_chars.len();

    // Loop through password input string by character
    for c in pass.chars() {
        // If our key is smaller than pass and we have
        // looped through the entire key already, iterate
        // over key from the start again
        if i == key_len {
            i = 0;
        }

        // Shift password character by each key character and add
        // to output string.
        let int_char = c as u8 + key_chars[i] as u8;
        out.push(int_char as char);

        i = i+1;
    }

    return out;
}

// Take a string and a key to perform a shift cipher decryption and
// return the result.
pub fn decrypt_pass(pass: String, key: String) -> String {
    // Init output string
    let mut out = String::with_capacity(pass.len());

    // Counter for which character of the key we are on
    let mut i = 0;

    // Turn the key into a character vector and save
    // the vector length
    let key_chars: Vec<char> = key.chars().collect();
    let key_len = key_chars.len();

    // Loop through password input string by character
    for c in pass.chars() {

        // If our key is smaller than pass and we have
        // looped through the entire key already, iterate
        // over key from the start again
        if i == key_len {
            i = 0;
        }

        // Shift password character by each key character and add
        // to output string.
        let int_char = c as u8 - key_chars[i] as u8;
        out.push(int_char as char);
        i = i+1;
    }

    return out;
}