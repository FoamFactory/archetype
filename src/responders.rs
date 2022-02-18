use rocket::http::{Header, ContentType};
use rocket::Request;
use rocket::response::content::Json;
use rocket::response::status;

#[derive(Responder)]
pub enum RequestError {
    #[response(status = 401, content_type = "json")]
    BadRequest(Json<String>),

    #[response(status = 415, content_type = "json")]
    UnsupportedMediaType(Json<String>),
    // #[response(status = 404)]
    // NotFound(NamedFile, ContentType),
}

impl From<(u16, &str)> for RequestError {
    fn from(error_tuple: (u16, &str)) -> Self {
        let (code, message) = error_tuple;
        let req_err_msg: RequestErrorMessage = RequestErrorMessage {
            message: String::from(message)
        };

        let json_obj = serde_json::to_string(&req_err_msg).unwrap();
        match code {
            415 => RequestError::UnsupportedMediaType(Json(json_obj)),
            _ => RequestError::BadRequest(Json(json_obj))
        }
    }
}

#[derive(Serialize)]
pub struct RequestErrorMessage {
    pub message: String
}