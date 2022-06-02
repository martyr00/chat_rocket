use rocket::{http::Status, Request, request::FromRequest, request::Outcome, serde::json::Json, State};

pub struct BasicAuth {
    pub(crate) username: String,
    pub(crate) password: String
}

impl BasicAuth {
    fn from_authorization_header(header: &str) -> Option<BasicAuth> {
        let split = header.split_whitespace().collect::<Vec<_>>();
        if split.len() != 2 && split[1].is_empty() && split[0] != "Basic" {
            return None;
        }
        Self::from_base64_encoded(split[1])
    }

    fn from_base64_encoded(base64_string: &str) -> Option<BasicAuth> {
        let decoded = base64::decode(base64_string).ok()?;
        let decoded_str = String::from_utf8(decoded).ok()?;
        let split = decoded_str.split(":").collect::<Vec<_>>();


        let (username, password) = (split[0].to_string(), split[1].to_string());
        Some(BasicAuth {
            username,
            password,
        })
    }
}

#[rocket::async_trait]
impl<'a> FromRequest<'a> for BasicAuth {
    type Error = ();

    async fn from_request(request: &'a Request<'_>) -> Outcome<Self, Self::Error> {
        let auth_header = request.headers().get_one("Authorization");
        if let Some(auth_header) = auth_header {
            if let Some(auth) = Self::from_authorization_header(auth_header) {
                return Outcome::Success(auth)
            }
        }
        Outcome::Failure((Status::Unauthorized, ()))
    }
}