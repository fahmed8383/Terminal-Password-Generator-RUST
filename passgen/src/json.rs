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

    // Get pass.json path
    let build_path = get_directory();

    // Write passwords with the new password added to the pass.json file
    let file_content = serde_json::to_string(&json_vals).expect("Unable to turn JSON struct into string");
    let mut file = File::create(build_path).unwrap();
    file.write_all(&file_content.as_bytes()).expect("Unable to write to json file ");
}

// Deletes password entry with the corresponding name
pub fn delete_password(name: &str, mut json_vals: JSON) {

    // Init remove_pos to be outside of possible values it can get.
    // This value will always update since we make sure the name
    // exists before calling this function.
    let mut remove_pos = json_vals.passwords.len() + 1;

    // Enumerate through json_vals without consuming the values.
    for (i, password) in json_vals.passwords.iter().enumerate() {

        // Update the remove_pos when we find the password with the
        // correct name.
        if password.name == name {
            remove_pos = i;
        }
    }

    // Remove the corresponding password with the name from the list
    json_vals.passwords.remove(remove_pos);

    // Get pass.json path
    let build_path = get_directory();

    // Write back passwords with the corresponding password remvoed to the pass.json file
    let file_content = serde_json::to_string(&json_vals).expect("Unable to turn JSON struct into string");
    let mut file = File::create(build_path).unwrap();
    file.write_all(&file_content.as_bytes()).expect("Unable to write to json file ");
}

// Return a list of all saved password names
pub fn get_password_list() -> Vec<String> {

    // Init vector to save names in
    let mut pass_list = Vec::new();

    // Generate the JSON struct by parsing the json file
    let json_vals = parse_passwords();

    // Loop through all passwords
    // Consume the struct on iter.
    for password in json_vals.passwords.into_iter() {
        pass_list.push(password.name);
    }

    // Return password lisr
    return pass_list;
}

// Parse the json file into the JSON struct
pub fn parse_passwords() -> JSON {

    // Get pass.json path
    let build_path = get_directory();

    // Open and read file into a string
    let mut file = File::open(build_path).unwrap();
    let mut file_content = String::new();
    file.read_to_string(&mut file_content).unwrap();

    // Parse json string into the JSON struct
    let json_vals: JSON = serde_json::from_str(&file_content).expect("Unable to read from JSON file");

    // Return the struct
    return json_vals;
}

// Check if the PASSGEN_PATH variable is set during
// compile time, if it is change file path to the
// variable
fn get_directory() -> String {
    let mut build_path;

    match option_env!("PASSGEN_PATH") {
        Some(val) => {
            build_path = String::from(val);
            build_path.push_str("pass.json");
        }
        None => {
            build_path = String::from("pass.json");
        }
    }

    return build_path;
}