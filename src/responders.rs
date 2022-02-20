use rocket::http::{Header, ContentType};
use rocket::Request;
use rocket::response::content::Json;
use rocket::response::status;

pub trait JsonRetriever {
  fn get_json(&self) -> &Json<String>;
}

#[derive(Debug, Responder)]
pub enum RequestError {
    #[response(status = 400, content_type = "json")]
    BadRequest(Json<String>),

    #[response(status = 403, content_type = "json")]
    Forbidden(Json<String>),

    #[response(status = 404, content_type = "json")]
    NotFound(Json<String>),

    #[response(status = 415, content_type = "json")]
    UnsupportedMediaType(Json<String>),
}

impl JsonRetriever for RequestError {
    fn get_json(&self) -> &Json<String> {
        match self {
            RequestError::NotFound(j) => j,
            RequestError::BadRequest(j) => j,
            RequestError::Forbidden(j) => j,
            RequestError::UnsupportedMediaType(j) => j,
        }
    }
}

impl From<(u16, &str)> for RequestError {
    fn from(error_tuple: (u16, &str)) -> Self {
        let (code, message) = error_tuple;
        let req_err_msg: RequestErrorMessage = RequestErrorMessage {
            message: String::from(message)
        };

        let json_obj = serde_json::to_string(&req_err_msg).unwrap();
        match code {
            403 => RequestError::Forbidden(Json(json_obj)),
            404 => RequestError::NotFound((Json(json_obj))),
            415 => RequestError::UnsupportedMediaType(Json(json_obj)),
            _ => RequestError::BadRequest(Json(json_obj))
        }
    }
}

#[derive(Serialize)]
pub struct RequestErrorMessage {
    pub message: String
}