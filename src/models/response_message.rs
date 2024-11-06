use serde::Serialize;

#[derive(Serialize)]
pub struct ResponseMessage {
    pub message: String,
}
