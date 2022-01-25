use rocket::http::Status;
use rocket::serde::{json::Json, Serialize};
use rocket::Request;

#[derive(Serialize)]
pub struct ErrorResponse<'a> {
    r#type: &'static str,
    code: u16,
    message: &'a str,
}

impl<'a> ErrorResponse<'a> {
    fn new(code: u16, message: &'a str) -> Self {
        ErrorResponse {
            r#type: "error",
            code,
            message,
        }
    }
}

#[catch(default)]
pub fn default_catcher(status: Status, _request: &Request) -> Json<ErrorResponse<'static>> {
    Json::from(ErrorResponse::new(
        status.code,
        match status.reason() {
            Some(reason) => reason,
            _ => "Unknown error",
        },
    ))
}
