use std::env;

// Import modules
mod print;
mod process;


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
                process::generate(args_len, &args);
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
        println!("Here is your password");
    }
}