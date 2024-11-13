//! # Next Era Utils
//!
//! Next Era Solutions Utilities for Rust.

pub mod models;
pub mod parser;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn testing_models() {
        let response = models::response_message::ResponseMessage {
            message: String::from("Hello"),
        };
        assert_eq!(response.message, String::from("Hello"));
    }

    #[test]
    fn testing_parser() {
        use crate::parser::OptionParserExtensions;

        let test: Option<&str> = Some("200");
        let final_result : Option<i32> = Some(200);
        let result = test.to_i32();
        assert_eq!(result, final_result);

        let test: Option<&str> = Some("hello");
        let result = test.to_i32();
        assert_eq!(result, None);
    }
}
