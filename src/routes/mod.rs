use bcrypt::{verify, BcryptResult};
use mongodb::bson::oid::ObjectId;
use rocket::futures::future::err;
use rocket::State;
use rocket::{http::Status, serde::json::Json};
use serde::{Deserialize, Serialize};
use std::future::Future;

use crate::database;
use crate::model::User;

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

#[derive(Debug, Serialize, Deserialize)]
pub struct MessageDBO {
    pub body: String,
    pub to: String,
    pub from: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MessageDBOId {
    pub body: String,
    pub to: ObjectId,
    pub from: ObjectId,
}

#[post("/chat/user", data = "<form>", format = "json")]
pub async fn post_new_item(
    mut form: Option<Json<UserDboPassUser>>,
    database: &State<database::MongoDB>,
) -> Result<Status, Status> {
    match form {
        Some(ref mut form) => {
            if get_is_valid_user_data(&form, database).await {
                match database.create_new_acc(form).await {
                    Ok(_) => Ok(Status::Ok),
                    Err(_) => Err(Status::InternalServerError),
                }
            } else {
                Err(Status::BadRequest)
            }
        }
        None => Err(Status::BadRequest),
    }
}

#[post("/chat/user/login", data = "<form>", format = "json")]
pub async fn post_login(
    mut form: Option<Json<UserDboPassUser>>,
    database: &State<database::MongoDB>,
) -> Result<Status, Status> {
    match form {
        Some(ref mut form) => match login(form.username.clone(), database).await {
            Ok(_) => Err(Status::Unauthorized),
            Err(result) => match verify(&form.password, &*result.password) {
                Ok(true) => Ok(Status::Ok),
                Ok(false) => Err(Status::Unauthorized),
                Err(_) => Err(Status::InternalServerError),
            },
        },
        None => Err(Status::Unauthorized),
    }
}

#[get("/chat/user")]
pub async fn get_all_acc(
    database: &State<database::MongoDB>,
) -> Result<Json<Vec<UserDboIdUser>>, Status> {
    return match database.get_all_items().await {
        Ok(users) => Ok(Json(users)),
        Err(error) => {
            println!("----------------");
            println!("error: {:?}", error);
            println!("----------------");
            Err(Status::InternalServerError)
        }
    };
}

#[post("/chat/message", data = "<form>", format = "json")]
pub async fn post_new_message(
    mut form: Option<Json<MessageDBO>>,
    database: &State<database::MongoDB>,
) -> Result<Status, Status> {
    match form {
        Some(ref mut form) => match object_id_parse_str(form.to.clone()) {
            Ok(to_id) => match object_id_parse_str(form.from.clone()) {
                Ok(from_id) => {
                    if get_is_valid_message_data(form.body.clone()).await {
                        let result = MessageDBOId {
                            body: form.body.clone(),
                            to: to_id,
                            from: from_id,
                        };
                        match database.post_message(result).await {
                            Ok(_) => Ok(Status::Ok),
                            Err(_) => Err(Status::InternalServerError),
                        }
                    } else {
                        Err(Status::BadRequest)
                    }
                }
                Err(_) => Err(Status::BadRequest),
            },
            Err(_) => Err(Status::BadRequest),
        },
        None => Err(Status::BadRequest),
    }
}

#[get("/chat/message/preview/<id>")]
pub async fn get_all_preview(
    id: String,
    database: &State<database::MongoDB>,
) -> Result<Json<Vec<UserDboIdUser>>, Status> {
    match object_id_parse_str(id) {
        Ok(id) => {
            match database.find_all_info_for_preview(id).await {
                Ok(user_from) => Ok(Json(user_from)),
                Err(_) => { Err(Status::NotFound) }
            }
        }
        Err(_) => { Err(Status::BadRequest) }
    }
}

fn object_id_parse_str(id_str: String) -> Result<ObjectId, String> {
    match ObjectId::parse_str(id_str) {
        Ok(to_id) => Ok(to_id),
        Err(error) => Err(format!("{}", error)),
    }
}

async fn get_is_valid_message_data(body: String) -> bool {
    return if !body.is_empty() {
        if body.len() < 200 {
            true
        } else {
            false
        }
    } else {
        false
    };
}

async fn get_is_valid_user_data(
    user: &UserDboPassUser,
    database: &State<database::MongoDB>,
) -> bool {
    let username = user.username.clone();
    let password = user.password.clone();

    return if !username.is_empty() && !password.is_empty() {
        if username.len() < 12 && password.len() >= 8 && password.len() < 18 {
            match login(username, database).await {
                Ok(true) => true,
                Ok(false) => false,
                Err(_) => false,
            }
        } else {
            false
        }
    } else {
        false
    };
}

async fn login(username: String, database: &State<database::MongoDB>) -> Result<bool, User> {
    match database.get_data_one_user(username).await {
        Ok(user_login) => match user_login {
            Some(result) => Err(result),
            None => Ok(true),
        },
        Err(_) => Ok(false),
    }
}
