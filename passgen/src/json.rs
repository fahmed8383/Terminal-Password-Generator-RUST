use serde::Deserialize;
use serde::Serialize;

use std::fs::File;
use std::io::Write;
use std::io::Read;

#[derive(Debug, Deserialize, Serialize)]
pub struct JSON {
    passwords: Vec<Passwords>
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Passwords {
    name: String,
    pass: String,
}

// Check if JSON struct contains the corresponding password name.
pub fn pass_exists(name: &str, json_vals: &JSON) -> bool {

    // Loop through json_vals without consuming the values.
    for password in json_vals.passwords.iter() {
        if password.name == name {
            return true;
        }
    }

    return false;
}

// Return the corresponding password for the password name in json_vals
pub fn get_password(name: &str, json_vals: JSON) -> String {

    // Consume the struct on iter.
    for password in json_vals.passwords.into_iter() {
        if password.name == name {
            return password.pass;
        }
    }

    return String::from("No such password name exists");
}

// Adds a password with the corresponding name and value
pub fn add_password(name: &str, pass: &str, mut json_vals: JSON) {

    // Create a new passwords struct to save the new password entry and add it to the JSON.passwords
    // vector.
    let new_pass: Passwords = Passwords { name: String::from(name), pass: String::from(pass) };
    json_vals.passwords.push(new_pass);

    // Write passwords with the new password added to the pass.json file
    let file_content = serde_json::to_string(&json_vals).expect("Unable to turn JSON struct into string");
    let mut file = File::create("pass.json").unwrap();
    file.write_all(&file_content.as_bytes()).expect("Unable to write to json file ");
}

// Parse the json file into the JSON struct
pub fn parse_passwords() -> JSON {

    // Open and read file into a string
    let mut file = File::open("pass.json").unwrap();
    let mut file_content = String::new();
    file.read_to_string(&mut file_content).unwrap();

    // Parse json string into the JSON struct
    let json_vals: JSON = serde_json::from_str(&file_content).expect("Unable to read from JSON file");

    // Return the struct
    return json_vals;
}