use std::env;

// Import modules
mod print;
mod json;
mod random;
mod encryption;
mod authenticate;

// Main function
fn main() {

    // Save command line arguments in a vector
    let args: Vec<String> = env::args().collect();

    // Store the number of command line arguments
    let args_len = args.len();

    // If we do not have the correct number of command line arguments exit
    if args_len == 1 || args_len > 3 {
        print::usage();
        return;
    }

    let arg_1 = &args[1];

    // Check to see if the first argument is a flag
    if arg_1.chars().nth(0).unwrap() == '-' {
                
        // Match the flag to run the appropriate code
        match &arg_1[..] {
            "-g" => {
                // Check to make sure we have a valid arg2
                if valid_arg2(args_len, &args) {

                    // Generate the JSON struct by parsing the json file
                    let json_vals: json::JSON = json::parse_passwords();

                    // If password already exists return error.
                    // json_vals Struct is passed by borrowing.
                    if json::pass_exists(&args[2], &json_vals) {
                        println!("Password with name {} already exists", &args[2]);
                        return;
                    }

                    // Get encryption key from user, ask for key twice to confirm.
                    let key = authenticate::get_key_confirm();

                    // If both keys that are entered do not match print error
                    // and exit.
                    if key == String::from("Passwords do not match") {
                        println!("Error: Keys do not match. No password generated");
                        return;
                    }
                    
                    // Generate new password string and output it
                    // to terminal
                    let pass = random::gen_random_string();
                    println!("{}", pass);
                    
                    // Encrypt password with basic shift cipher
                    let encrypted = encryption::encrypt_pass(pass, key);

                    // Add encrypted password to pass.json.
                    // json_vals Struct is owned and consumed.
                    json::add_password(&args[2], &encrypted, json_vals);
                }
            }
            "-flags" => {
                print::flags();
            }
            _ => {
                print::flags();
            }
        }
    }

    // Otherwise output the password for the specified name (default flag)
    else {

        // Generate the JSON struct by parsing the json file
        let json_vals: json::JSON = json::parse_passwords();

        // If password does not exist return error.
        // json_vals Struct is passed by borrowing.
        if !json::pass_exists(arg_1, &json_vals) {
            println!("No such password name exists");
            return;
        }
        
        // Get decryption key from user.
        let key = authenticate::get_key();

        // Get the password for the appropriate name from json file.
        // json_vals Struct is owned and consumed.
        let pass = json::get_password(arg_1, json_vals);

        // Decrypt password with basic shift cipher and print decrypted
        // password to the terminal.
        let decrypted = encryption::decrypt_pass(pass, key);
        println!("{}", decrypted);
    }
}

// Check to make sure the second argument of certain flags is valid
pub fn valid_arg2(args_len: usize, args: &Vec<String>) -> bool {

    // To have a second argument we need 3 arguments
    if args_len != 3 {
        print::usage();
        return false;
    }

    let arg_2 = &args[2];

    // Check to make sure the name of the password does not start with a -
    if arg_2.chars().nth(0).unwrap() == '-' {
        println!("password name cannot start with -");
        return false;
    }

    return true;
}