use bcrypt::verify;
use rocket::{http::Status, serde::json::Json, State};
use uuid::Uuid;

use crate::database;
use crate::routes::{get_is_valid_user_data, login, RegistrationResponse, UserDboPassUser};

// Bearer
// match get one header Authorization -> (Bearer 'TOKEN')
//     Some(header) => {
//         let array_header_val = header.split(" ")
//             if array_header_val[1].is_empty {return Err(Status::401)}
//             else {
//                 array_header_val[1].parse_from_JWD() => struct { user_id: 'ObjectId' }
//                 if find user in DB by user_id {
//                     return Ok(Status::Ok)
//                 } else {
//                      return Err(Status::401)
//                 }
//             }
//         },
//     None(_) => return Err(Status::401)

#[post("/registration", data = "<form>", format = "json")]
pub async fn post_registration(
    form: Option<Json<UserDboPassUser>>,
    database: &State<database::MongoDB>,
) -> Result<Json<RegistrationResponse>, Status> {
    match form {
        Some(form) => {
            if get_is_valid_user_data(&form, database).await {
                match database.create_new_acc(form).await {
                    Ok(_) => Ok(Json(RegistrationResponse {
                        token: Uuid::new_v4().to_string(),
                        temp_token: Uuid::new_v4().to_string(),
                    })),
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
