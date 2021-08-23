use rand::Rng;

// Generate random string of random length
pub fn gen_random_string() -> String {
    // init random
    let mut rng = rand::thread_rng();
    
    // List of valid characters that can be used in the string
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            0123456789)(&#!";

    // Generate random number between 20 and 35
    let pass_len = rng.gen_range(20..35);

    // Generate random string of pass_len using our specified
    // characters.
    let password: String = (0..pass_len)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect();
    
    // Return the generated string
    return password;
}