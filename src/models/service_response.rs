use serde::Serialize;

/// ### Generic response model for project.
/// `message` :  your message.
///
/// ### Example
///
/// ```
/// use nextera_utils::models::service_response::ServiceResponse;
///
/// let res_msg = ServiceResponse{status_code: 200, message: String::from("Your message") };
/// assert_eq!(res_msg.status_code, 200);
/// assert_eq!(res_msg.message, String::from("Your message"));
/// ```
#[derive(Serialize)]
pub struct ServiceResponse {
    pub status_code: u16,
    pub message: String,
}
