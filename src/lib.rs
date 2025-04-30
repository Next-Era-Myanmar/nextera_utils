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
        let token = "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOjMsImV4cCI6MTczMjIwMDQ3NywiaXNzIjoiTmV4dCBFcmEgQXV0aGVudGljYWl0b24gU2VydmljZSIsImF1ZCI6Ik5FWFQgRVJBIFVTRVIifQ.dSFOwqIq_FtTTU1GuB7KVROgQP6sjtfWRLtozG-JrR4";
        let secret =
            "ACCESS_SECRET_2024!@#super_secure_random_string_1234567890ABCDEFGHIJKLMNOPQRSTUVWXYZ";
        let audience = "NEXT ERA USER";
        match validate_jwt(token, secret, audience) {
            Ok(_) => {
                panic!("Expired Token!")
            }
            Err(_) => {
                println!("Token is expired!");
                assert_eq!(true, true)
            }
        };
        match get_user_id_from_token(token) {
            Ok(result) => {
                assert_eq!(result, 3)
            }
            Err(_) => {
                panic!("Failed to get user id")
            }
        }
        match get_jwt_claims_from_token(token) {
            Ok(result) => {
                assert_eq!(result.sub, 3);
                assert_eq!(result.iss, String::from("Next Era Authenticaiton Service"));
                assert_eq!(result.exp, 1732200477usize);
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
