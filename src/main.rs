#[macro_use]
extern crate rocket;

mod database;
mod model;
mod routes;

use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Header;
use rocket::{serde::json::Json, serde::Serialize, Request, Response};
use routes::*;

pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "POST, GET, PATCH, OPTIONS",
        ));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}

#[launch]
async fn rocket() -> _ {
    rocket::build()
        .attach(CORS)
        .attach(database::init().await)
        .mount(
            "/api/v1",
            routes![
                post_new_item,
                get_all_acc,
                post_login,
                post_new_message,
                get_all_preview,
                get_all_message_from_to
            ],
        )
        .register(
            "/",
            catchers![
                not_found,
                forbidden,
                unauthorized,
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

#[catch(401)]
fn unauthorized() -> Json<ServerError> {
    Json(ServerError {
        title: "Unauthorized".to_string(),
        desc: "he request requires user authentication.".to_string(),
    })
}
