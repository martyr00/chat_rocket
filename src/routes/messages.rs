use rocket::{http::Status, serde::json::Json, State};
use std::collections::HashSet;

use crate::database;
use crate::routes::{
    get_is_valid_message_data, object_id_parse_str, MessageDBO, MessageDBOId, MessageTwoUsers,
    UserDboIdUser,
};

#[get("/users")]
pub async fn get_all_acc(
    database: &State<database::MongoDB>,
) -> Result<Json<Vec<UserDboIdUser>>, Status> {
    return match database.get_all_items().await {
        Ok(users) => Ok(Json(users)),
        Err(_) => Err(Status::InternalServerError),
    };
}

#[post("/message", data = "<form>", format = "json")]
pub async fn post_new_message(
    form: Option<Json<MessageDBO>>,
    database: &State<database::MongoDB>,
) -> Result<Status, Status> {
    match form {
        Some(ref form) => match object_id_parse_str(form.to.clone()) {
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

#[get("/chats/<id>")]
pub async fn get_all_preview(
    id: String,
    database: &State<database::MongoDB>,
) -> Result<Json<HashSet<String>>, Status> {
    match object_id_parse_str(id) {
        Ok(id) => match database.find_all_info_for_preview(id).await {
            Ok(user_from) => Ok(Json(user_from)),
            Err(_) => Err(Status::NotFound),
        },
        Err(_) => Err(Status::BadRequest),
    }
}

#[get("/chats/<first_id>/<second_id>")]
pub async fn get_all_message_from_to(
    first_id: String,
    second_id: String,
    database: &State<database::MongoDB>,
) -> Result<Json<Vec<MessageTwoUsers>>, Status> {
    match object_id_parse_str(first_id) {
        Ok(to_id) => match object_id_parse_str(second_id) {
            Ok(from_id) => match database.get_massages_two_users(to_id, from_id).await {
                Ok(messages) => Ok(Json(messages)),
                Err(_) => Err(Status::NotFound),
            },
            Err(_) => Err(Status::BadRequest),
        },
        Err(_) => Err(Status::BadRequest),
    }
}
