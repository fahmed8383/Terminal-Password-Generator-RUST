use rpassword;

// Get encryption key from user twice and confirm if keys match
pub fn get_key_confirm() -> String{

    let pass = rpassword::read_password_from_tty(Some("Enter key to encrypt password with: ")).unwrap();
    let pass_confirm = rpassword::read_password_from_tty(Some("Re-enter key: ")).unwrap();

    if pass == pass_confirm {
        return pass;
    }

    return String::from("Passwords do not match");
}

// Get encryption key from user
pub fn get_key() -> String{

    let pass = rpassword::read_password_from_tty(Some("Enter key to decrypt password with: ")).unwrap();
    return pass;
}