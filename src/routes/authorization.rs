use base64::{decode, encode, DecodeError};
use bcrypt::verify;
use rocket::{http::Status, Request, request::FromRequest, request::Outcome, serde::json::Json, State};
use rocket::form::validate::len;
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

pub struct Authorization {
    //pub(crate) username: String,
    pub(crate) password: String
}

impl Authorization {
    fn from_authorization_header(header: &str) -> Option<Authorization> {
        let split = header.split_whitespace().collect::<Vec<_>>();
        if split.len() != 2 && split[1].is_empty() && split[0] != "Authorization" {
            return None;
        }
        Self::from_base64_encoded(split[1])
    }

    fn from_base64_encoded(base64_string: &str) ->Option<Authorization> {
        let decoded = base64::decode(base64_string).ok()?;
        let decoded_str = String::from_utf8(decoded).ok()?;
        let split = decoded_str.split(":::").collect::<Vec<_>>();

        if split.len() != 1 {
            return None;
        }

        //let (username, password) = (split[0].to_string(), split[1].to_string());
        Some(Authorization {
            //username,
            password: split[0].to_string(),
        })
    }
}

#[rocket::async_trait]
impl<'a> FromRequest<'a> for Authorization {
    type Error = ();

    async fn from_request(request: &'a Request<'_>) -> Outcome<Self, Self::Error> {
        let auth_header = request.headers().get_one("Authorization");
        if let Some(auth_header) = auth_header {
            if let Some(auth) = Self::from_authorization_header(auth_header) {
                return Outcome::Success(auth)
            }
        }
        Outcome::Failure((Status::Unauthorized, ()))
    }
}

#[post("/registration", data = "<form>", format = "json")]
pub async fn post_registration(
    form: Option<Json<UserDboPassUser>>,
    database: &State<database::MongoDB>,
) -> Result<Json<RegistrationResponse>, Status> {
    match form {
        Some(form) => {
            let base64_password = base64::encode(form.password.clone());
            //match  {
             //   Ok(token) => {
                    if get_is_valid_user_data(&form, database).await {
                        match database.create_new_acc(form).await {
                            Ok(_) => Ok(Json(RegistrationResponse {
                                token: base64_password,
                                //temp_token: Uuid::new_v4().to_string(),
                            })),
                            Err(_) => Err(Status::InternalServerError),
                        }
                    } else {
                        Err(Status::BadRequest)
                    }
                }
                //Err(error) => {
               //     println!("{}", error);
              //   Err(Status::BadRequest)},
          //  }}

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
