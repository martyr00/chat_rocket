use rocket::{http::Status, serde::json::Json};
use serde::{Deserialize, Serialize};
use rocket::State;

use crate::database;

#[derive(Debug, Serialize, Deserialize)]
pub struct UserDboPassUser {
    pub(crate) username: String,
    pub(crate) password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserDboIdUser {
    pub(crate) _id: String,
    pub(crate) username: String,
}

#[post("/chat/user", data = "<form>", format = "json")]
pub async fn post_new_item(
    mut form: Option<Json<UserDboPassUser>>,
    database: &State<database::MongoDB>,
) -> Result<Status, Status> {
    match form {
        Some(ref mut form) => {
            if get_is_valid_user_data(&form) {
                match database.create_new_acc(form).await {
                    Ok(_) => Ok(Status::Ok),
                    Err(_) => Err(Status::InternalServerError)
                }
            } else {
                Err(Status::BadRequest)
            }
        }
        None => Err(Status::BadRequest)
    }
}

#[get("/chat/user")]
pub async fn get_all_acc(
    database: &State<database::MongoDB>,
) -> Result<Json<Vec<UserDboIdUser>>, Status> {
    return match database.get_all_items().await {
        Ok(users_todo) => Ok(Json(users_todo)),
        Err(error) => {
            println!("----------------");
            println!("error: {:?}", error);
            println!("----------------");
            Err(Status::InternalServerError)
        }
    };
}

fn get_is_valid_user_data(user: &UserDboPassUser) -> bool {
    let username = &user.username;
    let password = &user.password;

    return if !username.is_empty() && !password.is_empty() {
        if username.len() < 12 && password.len() < 18{
            true
        } else {
            false
        }
    } else {
        false
    };
}
