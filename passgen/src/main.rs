use std::env;

// Import modules
mod print;
mod json;
mod random;

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
                    
                    // Generate new password string
                    let pass = random::gen_random_string();

                    // Add password to pass.json
                    json::add_password(&args[2], &pass);
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

        // Get the password for the appropriate name if it exists from json file
        let pass = json::get_password(arg_1);

        // If it does not exist print error and exit
        if pass == "No such password name exists" {
            println!("No such password name exists");
            return;
        }

        println!("{}", pass);
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