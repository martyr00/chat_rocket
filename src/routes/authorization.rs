use bcrypt::verify;
use rocket::{http::Status, serde::json::Json, State};

use uuid::Uuid;
use crate::database;

use crate::routes::{get_is_valid_user_data, login, Tokens, UserDboPassUser};

#[post("/registration", data = "<form>", format = "json")]
pub async fn post_registration(
    form: Option<Json<UserDboPassUser>>,
    database: &State<database::MongoDB>,
) -> Result<Json<Tokens>, Status> {
    match form {
        Some(form) => {
            if get_is_valid_user_data(&form, database).await {
                match database.create_new_acc(form).await {
                    Ok(_) => Ok(Json(Tokens {
                        token: Uuid::new_v4().to_string(),
                        temp_token: Uuid::new_v4().to_string()
                    }
                    )),
                    Err(_) => Err(Status::InternalServerError),
                }
            } else {
                Err(Status::BadRequest)
            }
        }
        None => Err(Status::BadRequest),
    }
}

#[post("/login", data = "<form>", format = "json")]
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