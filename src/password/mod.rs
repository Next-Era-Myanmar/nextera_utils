use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use rand::rngs::OsRng;

pub struct Password;

impl Password {
    /// ### Hashing password using argon2.
    ///
    /// ### Example
    ///
    /// ```
    /// use nextera_utils::password::Password;
    /// let password = String::from("Password");
    /// match Password::hash_password(password){
    ///     Ok(hashed_password)=>println!("{}" ,hashed_password),
    ///     Err(e)=>println!("Error: {}",e.to_string())
    /// };
    /// ```
    pub fn hash_password(password: String) -> Result<String, Box<dyn std::error::Error>> {
        // Generate a random salt
        let salt = SaltString::generate(&mut OsRng);

        // Configure Argon2
        let argon2 = Argon2::default();

        // Hash the password
        let password_hash = argon2
            .hash_password(password.as_str().as_bytes(), &salt)
            .unwrap();
        Ok(password_hash.to_string())
    }

    /// ### Verifying password that hashing with argon2.
    ///
    /// ### Example
    ///
    /// ```
    /// use nextera_utils::password::Password;
    /// let password = String::from("Password");
    /// match Password::hash_password(password.clone()){
    ///     Ok(hashed_password)=>{
    ///         match Password::verify_password(hashed_password,password){
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
    ) -> Result<bool, Box<dyn std::error::Error>> {
        // Parse the hash
        let parsed_hash = PasswordHash::new(hash.as_str()).unwrap();

        // Verify the password against the hash
        let argon2 = Argon2::default();
        Ok(argon2
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok())
    }
}
