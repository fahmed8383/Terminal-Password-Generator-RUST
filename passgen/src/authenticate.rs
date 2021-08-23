use std::env;
use rpassword;

// Get encryption key from user twice and confirm if keys match
pub fn get_key_confirm() -> String{

    // If key has been saved as an environment variable by user,
    // use that.
    if env::var("PASSGEN_KEY").is_ok() {
        return env::var("PASSGEN_KEY").unwrap();
    }

    let pass = rpassword::read_password_from_tty(Some("Enter key to encrypt password with: ")).unwrap();
    let pass_confirm = rpassword::read_password_from_tty(Some("Re-enter key: ")).unwrap();

    if pass == pass_confirm {
        return pass;
    }

    return String::from("Keys do not match. Please try again");
}

// Get encryption key from user
pub fn get_key() -> String{

    // If key has been saved as an environment variable by user,
    // use that.
    if env::var("PASSGEN_KEY").is_ok() {
        return env::var("PASSGEN_KEY").unwrap();
    }

    let pass = rpassword::read_password_from_tty(Some("Enter key to decrypt password with: ")).unwrap();
    return pass;
}