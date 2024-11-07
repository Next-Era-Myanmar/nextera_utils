use serde::Serialize;

/// ### Generic response model for project.
/// `message` :  your message.
///
/// ### Example
///
/// ```
/// use nextera_utils::models::response_message::ResponseMessage;
///
/// let res_msg = ResponseMessage{ message: String::from("Your message") };
/// assert_eq!(res_msg.message, String::from("Your message"));
/// ```
#[derive(Serialize)]
pub struct ResponseMessage {
    pub message: String,
}
