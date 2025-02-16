use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use bcrypt::{hash, DEFAULT_COST};
use rand::distributions::Alphanumeric;
use rand::rngs::OsRng;
use rand::Rng;

pub struct Password;

pub enum PasswordHasherType {
    Argon2,
    Bcrypt,
}

impl Password {
    /// ### Hashing password using argon2.
    ///
    /// ### Example
    ///
    /// ```
    /// use nextera_utils::password::{Password, PasswordHasherType};
    /// let password = String::from("Password");
    /// match Password::hash_password(password,PasswordHasherType::Argon2){
    ///     Ok(hashed_password)=>println!("{}" ,hashed_password),
    ///     Err(e)=>println!("Error: {}",e.to_string())
    /// };
    /// ```
    pub fn hash_password(
        password: String,
        password_hasher_type: PasswordHasherType,
    ) -> Result<String, String> {
        match password_hasher_type {
            PasswordHasherType::Argon2 => {
                // Generate a random salt
                let salt = SaltString::generate(&mut OsRng);

                // Configure Argon2
                let argon2 = Argon2::default();

                // Hash the password
                let result = argon2.hash_password(password.as_str().as_bytes(), &salt);
                match result {
                    Ok(password_hash) => Ok(password_hash.to_string()),
                    Err(e) => Err(e.to_string()),
                }
            }
            PasswordHasherType::Bcrypt => {
                let result = hash(password, DEFAULT_COST);
                match result {
                    Ok(password_hash) => Ok(password_hash),
                    Err(e) => Err(e.to_string()),
                }
            }
        }
    }

    /// ### Verifying password that hashing with argon2.
    ///
    /// ### Example
    ///
    /// ```
    /// use nextera_utils::password::{Password, PasswordHasherType};
    /// let password = String::from("Password");
    /// match Password::hash_password(password.clone(),PasswordHasherType::Argon2){
    ///     Ok(hashed_password)=>{
    ///         match Password::verify_password(hashed_password,password,PasswordHasherType::Argon2){
    ///             Ok(result)=>println!("result : {}" ,result),
    ///             Err(e)=>println!("Error: {}",e.to_string())
    ///         };
    ///     },
    ///     Err(e)=>println!("{}",e.to_string())
    /// };
    /// ```
    pub fn verify_password(
        hash: String,
        password: String,
        password_hasher_type: PasswordHasherType,
    ) -> Result<bool, String> {
        match password_hasher_type {
            PasswordHasherType::Argon2 => {
                // Parse the hash
                let parsed_hash = PasswordHash::new(hash.as_str()).unwrap();

                // Verify the password against the hash
                let argon2 = Argon2::default();
                let result = argon2
                    .verify_password(password.as_bytes(), &parsed_hash).is_ok();
                Ok(result)
            }
            PasswordHasherType::Bcrypt => {
                let result = bcrypt::verify(password, hash.as_str());
                match result {
                    Ok(is_valid) => Ok(is_valid),
                    Err(e) => Err(e.to_string()),
                }
            }
        }
    }
}

/// Generates a strong password of specified length `n`
/// The password contains uppercase, lowercase, digits, and special characters.
pub fn generate_strong_password(n: usize) -> String {
    // Define character groups
    const SPECIAL_CHARS: &str = "!@#$%^&*()_+{}[]:;<>,.?/|~`";

    // Ensure we have enough characters for a strong password
    if n < 4 {
        panic!("Password length must be at least 4 to ensure complexity.");
    }

    let mut rng = rand::thread_rng();

    // Generate at least one character from each group
    let mut password = vec![
        (rng.sample(Alphanumeric) as char).to_ascii_lowercase(), // Lowercase
        (rng.sample(Alphanumeric) as char).to_ascii_uppercase(), // Uppercase
        rng.gen_range('0'..='9'),                                // Digit
        SPECIAL_CHARS
            .chars()
            .nth(rng.gen_range(0..SPECIAL_CHARS.len()))
            .unwrap(), // Special character
    ];

    // Fill the rest of the password with random alphanumeric or special characters
    password.extend((0..n - 4).map(|_| {
        let choice = rng.gen_range(0..3);
        match choice {
            0 => (rng.sample(Alphanumeric) as char).to_ascii_lowercase(), // Lowercase
            1 => (rng.sample(Alphanumeric) as char).to_ascii_uppercase(), // Uppercase
            _ => SPECIAL_CHARS
                .chars()
                .nth(rng.gen_range(0..SPECIAL_CHARS.len()))
                .unwrap(), // Special
        }
    }));

    // Shuffle the password to avoid predictable patterns
    use rand::seq::SliceRandom;
    password.shuffle(&mut rng);

    // Collect the password into a String and return
    password.into_iter().collect()
}
