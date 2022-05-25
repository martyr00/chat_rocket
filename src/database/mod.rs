mod private;
extern crate bcrypt;

use crate::database::private::DB;
use bcrypt::{hash, verify, BcryptResult, DEFAULT_COST};

use crate::model::User;
use crate::{UserDboIdUser, UserDboPassUser};
use mongodb::{bson, bson::oid::ObjectId, options::ClientOptions, Client, Database};
use rocket::{fairing::AdHoc, futures::TryStreamExt};

pub struct MongoDB {
    database: Database,
}

impl MongoDB {
    fn new(database: Database) -> Self {
        MongoDB { database }
    }
    pub async fn create_new_acc(&self, user: &mut UserDboPassUser) -> mongodb::error::Result<()> {
        let collection = self.database.collection::<User>("chat");

        match hash(&user.password, 4) {
            Ok(hash_password) => {
                collection
                    .insert_one(
                        User {
                            _id: ObjectId::new(),
                            username: user.username.clone(),
                            password: hash_password,
                        },
                        None,
                    )
                    .await?;
                Ok(())
            }
            Err(_) => Ok(()),
        }
    }

    pub async fn get_all_items(&self) -> mongodb::error::Result<Vec<UserDboIdUser>> {
        let collection = self.database.collection::<User>("chat");

        let mut cursor = collection.find(None, None).await?;

        let mut users: Vec<UserDboIdUser> = vec![];
        while let Some(result) = cursor.try_next().await? {
            let customer_json = UserDboIdUser {
                _id: result._id.to_string(),
                username: result.username.to_string(),
            };
            users.push(customer_json);
        }

        Ok(users)
    }

    pub async fn post_login(&self, username: String) -> mongodb::error::Result<Option<User>> {
        let collection = self.database.collection::<User>("chat");
        Ok(collection
            .find_one(bson::doc! { "username": username }, None)
            .await?)
    }
}

pub async fn init() -> AdHoc {
    AdHoc::on_ignite("Connect to MongoDB cluster", |rocket| async {
        match connect().await {
            Ok(database) => rocket.manage(MongoDB::new(database)),
            Err(error) => {
                panic!("Cannot connect to MDB instance:: {:?}", error)
            }
        }
    })
}

async fn connect() -> mongodb::error::Result<Database> {
    let client_options = ClientOptions::parse(DB).await?;
    let client = Client::with_options(client_options)?;
    // Ping the server to see if you can connect to the cluster
    client
        .database("admin")
        .run_command(bson::doc! {"ping": 1}, None)
        .await?;

    println!("connected to DB");

    Ok(client.database("chat"))
}
