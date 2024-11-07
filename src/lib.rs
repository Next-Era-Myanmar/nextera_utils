//! # Next Era Utils
//!
//! Next Era Solutions Utilities for Rust.

pub mod models;

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
}
