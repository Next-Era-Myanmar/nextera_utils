//! # Next Era Utils
//!
//! Next Era Solutions Utilities for Rust.

pub mod jwt;
pub mod models;
pub mod parser;
pub mod password;
pub mod time;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::jwt::{get_jwt_claims_from_token, get_user_id_from_token, validate_jwt};
    use crate::password::PasswordHasherType;
    use crate::time::Time;

    #[test]
    fn testing_models() {
        // Response Message
        let response = models::response_message::ResponseMessage {
            message: String::from("Hello"),
        };
        assert_eq!(response.message, String::from("Hello"));

        // Response Data
        let res_data = models::response_data::ResponseData::<i32> {
            data: vec![1, 2, 3],
            total: 3,
        };
        assert_eq!(res_data.data.len(), 3);
        assert_eq!(res_data.total, 3);

        // Service Response
        let res_data = models::service_response::ServiceResponse {
            status_code: 200,
            message: String::from("Hello"),
        };
        assert_eq!(res_data.status_code, 200);
        assert_eq!(res_data.message, String::from("Hello"));

        // Cache Data
        let res_data = models::cache_data::CacheData::<i32> {
            data: vec![1, 2, 3],
            total: 3,
        };
        assert_eq!(res_data.data.len(), 3);
        assert_eq!(res_data.total, 3);
    }

    #[test]
    fn testing_parser() {
        use crate::parser::OptionParserExtensions;
        use crate::parser::ParserExtensions;

        let test: Option<&str> = Some("200");
        let final_result: Option<i32> = Some(200);
        let result = test.to_opt_i32();
        assert_eq!(result, final_result);

        let test: Option<&str> = Some("hello");
        let result = test.to_opt_i32();
        assert_eq!(result, None);

        let test: String = String::from("200");
        let final_result: Option<u16> = Some(200);
        let result = test.to_opt_u16();
        assert_eq!(result, final_result);

        let test: String = String::from("Hello");
        let result = test.to_opt_u16();
        assert_eq!(result, None);
    }

    #[test]
    fn testing_password() {
        use crate::password::Password;

        let password: String = String::from("Password");
        let wrong_password: String = String::from("Passwords");

        match Password::hash_password(password.clone(), PasswordHasherType::Argon2) {
            Ok(hashed_password) => {
                match Password::verify_password(
                    hashed_password.clone(),
                    password,
                    PasswordHasherType::Argon2,
                ) {
                    Ok(result) => assert_eq!(result, true),
                    Err(_) => panic!("Failed to verify password"),
                }
                match Password::verify_password(
                    hashed_password,
                    wrong_password,
                    PasswordHasherType::Argon2,
                ) {
                    Ok(result) => assert_eq!(result, false),
                    Err(_) => panic!("Failed to verify password"),
                }
            }
            Err(_) => panic!("Failed to hashed password"),
        }
    }

    #[test]
    fn testing_time() {
        let current_utc_time = Time::get_utc();
        let default_time = String::from("1");
        assert_ne!(current_utc_time.to_string(), default_time);
        let current_time = Time::get_now();
        let default_time1 = String::from("1");
        assert_ne!(current_time.to_string(), default_time1);
        let timezones = Time::get_supported_timezones();
        let failed = timezones.is_empty();
        assert!(!failed);
        let mm_timezone = String::from("UTC+06:30");
        let checked_timezone = Time::validate_timezone(&mm_timezone);
        assert_eq!(checked_timezone, mm_timezone);
        let utc_datetime = Time::get_utc();
        let converted_datetime = Time::convert_timezone(utc_datetime, mm_timezone);
        assert_ne!(converted_datetime.to_string(), utc_datetime.to_string());
    }

    #[test]
    fn testing_jwt() {
        let user_id = 1;
        let org_id = 1;
        let secret = "YourOrgSecret";
        let audience = "NEXTERA USER";
        // Generate Test
        let t = match jwt::generate_jwt(user_id, org_id, secret, 86400, "Next Era Authentication Service", audience) {
            Ok((token, _)) => {
                assert!(token.clone().len() > 0);
                token
            }
            Err(e) => {
                panic!("Failed to generate JWT: {}", e);
            }
        };
        let token = t.as_str();
        // Validate Test
        match validate_jwt(token, secret) {
            Ok(result) => {
                assert_eq!(result.claims.sub, user_id);
                assert_eq!(result.claims.org, org_id);
            }
            Err(e) => {
                panic!(e.to_string())
            }
        };
        match get_user_id_from_token(token) {
            Ok(result) => {
                assert_eq!(result, user_id)
            }
            Err(_) => {
                panic!("Failed to get user id")
            }
        }
        match get_jwt_claims_from_token(token) {
            Ok(result) => {
                assert_eq!(result.sub, user_id);
                assert_eq!(result.suid, String::from("Next Era Authentication Service"));
                assert_eq!(result.org, org_id);
                assert_eq!(result.aud, audience.to_string());
            }
            Err(_) => {
                panic!("Failed to get user id")
            }
        }
    }

    #[test]
    fn test_generate_strong_password_length() {
        let length = 12;
        let password = password::generate_strong_password(length);
        assert_eq!(password.len(), length);
    }

    #[test]
    fn test_generate_strong_password_complexity() {
        let length = 12;
        let password = password::generate_strong_password(length);

        // Ensure password contains at least one lowercase letter
        assert!(password.chars().any(|c| c.is_lowercase()));

        // Ensure password contains at least one uppercase letter
        assert!(password.chars().any(|c| c.is_uppercase()));

        // Ensure password contains at least one digit
        assert!(password.chars().any(|c| c.is_digit(10)));

        // Ensure password contains at least one special character
        let special_chars = "!@#$%^&*()_+{}[]:;<>,.?/|~`";
        assert!(password.chars().any(|c| special_chars.contains(c)));
    }

    #[test]
    #[should_panic(expected = "Password length must be at least 4 to ensure complexity.")]
    fn test_generate_strong_password_too_short() {
        password::generate_strong_password(3);
    }
}
