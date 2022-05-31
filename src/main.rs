#[macro_use]
extern crate rocket;

use crate::authorization::{post_login, post_registration};
use crate::errors_catch::{bad_request, forbidden, internal_sever_error, not_found, unauthorized};
use crate::messages::{get_all_acc, get_all_message_from_to, get_all_preview, post_new_message};
use crate::routes::authorization;
use crate::routes::errors_catch;
use crate::routes::messages;

mod database;
mod model;
mod routes;

#[launch]
async fn rocket() -> _ {
    rocket::build()
        .attach(database::init().await)
        //.attach(database::init().await)
        .mount(
            "/api/v1",
            routes![
                post_registration,
                post_login,
                get_all_acc,
                post_new_message,
                get_all_preview,
                get_all_message_from_to,
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
