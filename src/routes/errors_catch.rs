use rocket::serde::json::Json;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ServerError {
    title: String,
    desc: String,
}

#[catch(500)]
pub fn internal_sever_error() -> Json<ServerError> {
    Json(ServerError {
        title: "Internal Server Error".to_string(),
        desc: "The server encountered an internal error while processing this request".to_string(),
    })
}

#[catch(400)]
pub fn bad_request() -> Json<ServerError> {
    Json(ServerError {
        title: "bad_request".to_string(),
        desc: "The server was unable to understand the request due to invalid syntax".to_string(),
    })
}

#[catch(403)]
pub fn forbidden() -> Json<ServerError> {
    Json(ServerError {
        title: "Forbidden".to_string(),
        desc: "You are denied access".to_string(),
    })
}

#[catch(404)]
pub fn not_found() -> Json<ServerError> {
    Json(ServerError {
        title: "Nof found".to_string(),
        desc: "Nof found".to_string(),
    })
}

#[catch(401)]
pub fn unauthorized() -> Json<ServerError> {
    Json(ServerError {
        title: "Unauthorized".to_string(),
        desc: "he request requires user authentication.".to_string(),
    })
}
