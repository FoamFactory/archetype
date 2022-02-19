use std::net::IpAddr;
use rocket::http::Status;
use rocket::Request;
use rocket::request::{FromRequest, Outcome};
use crate::{responders, util};
use crate::responders::RequestError;

// pub struct AllowedHosts<'r>(&'r str);
pub struct AllowedHosts;

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AllowedHosts {
    type Error = RequestError;
    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let req_error = RequestError::from((403, "Host not in allowed list"));

        let allowed_hosts = util::get_allowed_hosts_from_environment();

        if request.remote().is_none() {
            return Outcome::Failure((Status::from_code(403).unwrap(), req_error));
        }

        let remote_machine = request.remote().unwrap();
        if allowed_hosts.contains(&remote_machine.ip()) {
            return Outcome::Success(AllowedHosts {});
        }

        Outcome::Failure((Status::from_code(403).unwrap(), req_error))
    }
}