#[macro_use] extern crate rocket;

mod database;
mod model;
mod routes;

use rocket::{serde::json::Json, serde::Serialize};
use routes::*;

#[launch]
async fn rocket() -> _ {
    rocket::build()
        .attach(database::init().await)
        .mount(
            "/api/v1",
            routes![
            ],
        )
        .register(
            "/",
            catchers![
                not_found,
                forbidden,
                unprocessable_entity,
                bad_request,
                internal_sever_error
            ],
        )
}

#[derive(Debug, Serialize)]
struct ServerError {
    title: String,
    desc: String,
}

#[catch(500)]
fn internal_sever_error() -> Json<ServerError> {
    Json(ServerError {
        title: "Internal Server Error".to_string(),
        desc: "The server encountered an internal error while processing this request".to_string(),
    })
}

#[catch(400)]
fn bad_request() -> Json<ServerError> {
    Json(ServerError {
        title: "bad_request".to_string(),
        desc: "The server was unable to understand the request due to invalid syntax".to_string(),
    })
}

#[catch(403)]
fn forbidden() -> Json<ServerError> {
    Json(ServerError {
        title: "Forbidden".to_string(),
        desc: "You are denied access".to_string(),
    })
}

#[catch(404)]
fn not_found() -> Json<ServerError> {
    Json(ServerError {
        title: "Nof found".to_string(),
        desc: "Nof found".to_string(),
    })
}

#[catch(422)]
fn unprocessable_entity() -> Json<ServerError> {
    Json(ServerError {
        title: "Unprocessable Entity".to_string(),
        desc: "The request was well-formed but was unable to be followed due to semantic api."
            .to_string(),
    })
}
