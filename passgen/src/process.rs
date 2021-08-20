use crate::print;

// Generate a new password with the specified name;
// Passing args vector by reference
pub fn generate(args_len: usize, args: &Vec<String>) {
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