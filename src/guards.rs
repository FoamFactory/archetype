use rocket::http::Status;
use rocket::Request;
use rocket::request::{FromRequest, Outcome};
use crate::{RequestError, util};

pub struct AllowedHosts;

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AllowedHosts {
    type Error = RequestError;
    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let req_error = RequestError::from((403, "Unable to retrieve host from request"));

        if request.remote().is_none() {
            return Outcome::Failure((Status::from_code(403).unwrap(), req_error));
        }

        let remote_machine = request.remote().unwrap();
        let req_error = RequestError::from((403, format!("Host {} not in allowed list", &remote_machine.ip()).as_str()));
        if util::is_host_allowed(&remote_machine.ip()) {
            return Outcome::Success(AllowedHosts {})
        }

        Outcome::Failure((Status::from_code(403).unwrap(), req_error))
    }
}