// Prints message on how to correctly use the command if the wrong arguments are entered
pub fn usage() {
    println!("Invalid usage.\nTo get an existing password please enter the name that \
    the password is stored under.\nTo generate a new password, please enter the name \
    you want to store the password under alongside the -g flag\n\n\
    Example usage for getting existing password:\npassgen test\n\n\
    Example usage for generating a new password:\npassgen -g test\n\n\
    For a list of other flags that can be used, type:\n\
    passgen -flags");
}

// Print a list of all valid flags
pub fn flags() {
    println!("Invalid flag. The following flags can be used:\n\n\
    -g : Generate a new password\n\
    -flags: print a list of all flags\n");
}