use mongodb::bson::oid::ObjectId;
use rocket::State;
use serde::{Deserialize, Serialize};

use crate::database;
use crate::model::User;

pub(crate) mod authorization;
pub(crate) mod errors_catch;
pub(crate) mod messages;
pub(crate) mod is_authorization;

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
pub struct RegistrationResponse {
    pub(crate) token: String,
    //pub(crate) temp_token: String
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

#[derive(Debug, Serialize, Deserialize)]
pub struct MessageTwoUsers {
    pub body: String,
    pub to: String,
    pub from: String,
    pub time: String,
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

// BASIC
// get one header Authorization -> (Basic dGVzdEB0ZXN0LmNvbQ==:::cXdlcnR5)
// if header does not exist
//      return 401
// else
//      split string by space -> 'Basic', 'dGVzdEB0ZXN0LmNvbQ==:::cXdlcnR5'
//      get second item -> 'dGVzdEB0ZXN0LmNvbQ==:::cXdlcnR5'
//      split token by ":::" -> 'dGVzdEB0ZXN0LmNvbQ==', 'cXdlcnR5'
//      get first item -> 'dGVzdEB0ZXN0LmNvbQ=='
//          parse item from base64 -> 'test@test.com'
//          find user in DB by email
//      get second item -> 'cXdlcnR5'
//          parse item from base64 -> 'qwerty'
//          if user password equal parsed value
//              user authorized
//          else
//              return 401
//
//


