use std::env;

// Import functions from other files
mod print;


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
                generate(args_len, &args);
            }
            "-flags" => {
                print::flags();
            }
            _ => {
                print::flags();
                return;
            }
        }
    }

    // Otherwise output the password for the specified name (default flag)
    else {
        println!("Here is your password");
    }
}

// Generate a new password with the specified name;
// Passing args vector by reference
fn generate(args_len: usize, args: &Vec<String>) {
    // For the generate flag we need 3 arguments
    if args_len != 3 {
        print::usage();
        return;
    }

    let arg_2 = &args[2];

    // Check to make sure the name of the password does not start with a -
    if arg_2.chars().nth(0).unwrap() == '-' {
        println!("password name cannot start with -");
        return;
    }

    println!("Generating password");
}