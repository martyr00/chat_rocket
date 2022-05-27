mod private;
extern crate bcrypt;

use crate::database::private::DB;
use bcrypt::hash;
use chrono::{NaiveTime, Utc};
use std::collections::HashSet;

use crate::model::{Message, User};
use crate::{MessageDBOId, MessageTwoUsers, UserDboIdUser, UserDboPassUser};
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
        let collection = self.database.collection::<User>("user");

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
        let collection = self.database.collection::<User>("user");

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

    pub async fn get_data_one_user(
        &self,
        username: String,
    ) -> mongodb::error::Result<Option<User>> {
        let collection = self.database.collection::<User>("user");
        Ok(collection
            .find_one(bson::doc! { "username": username }, None)
            .await?)
    }

    pub async fn post_message(&self, message_dbo: MessageDBOId) -> mongodb::error::Result<()> {
        let collection = self.database.collection::<Message>("message");
        let time: NaiveTime = Utc::now().time();
        collection
            .insert_one(
                Message {
                    body: message_dbo.body.clone(),
                    to: message_dbo.to.clone(),
                    from: message_dbo.from.clone(),
                    time: time.to_string(),
                },
                None,
            )
            .await?;
        Ok(())
    }

    pub async fn find_all_info_for_preview(
        &self,
        id: ObjectId,
    ) -> mongodb::error::Result<HashSet<String>> {
        let collection_message = self.database.collection::<Message>("message");
        let collection_user = self.database.collection::<User>("user");

        let mut cursor_massage = collection_message
            .find(bson::doc! { "to": id }, None)
            .await?;

        let mut username_hashset = HashSet::<String>::new();

        while let Some(res_message) = cursor_massage.try_next().await? {
            let mut cursor_user = collection_user
                .find(bson::doc! { "_id": res_message.from }, None)
                .await?;
            while let Some(res_user) = cursor_user.try_next().await? {
                let username_json = res_user.username.to_string();

                username_hashset.insert(username_json);
            }
        }
        Ok(username_hashset)
    }

    pub async fn get_massages_two_users(
        &self,
        to_id: ObjectId,
        from_id: ObjectId,
    ) -> mongodb::error::Result<Vec<MessageTwoUsers>> {
        let collection_message = self.database.collection::<Message>("message");
        let collection_user = self.database.collection::<User>("user");

        let mut vec_messages = Vec::new();

        let mut cursor_massage = collection_message
            .find(bson::doc! { "to": to_id, "from": from_id }, None)
            .await?;

        while let Some(res_message) = cursor_massage.try_next().await? {
            let mut cursor_from_user = collection_user
                .find(bson::doc! { "_id": res_message.from }, None)
                .await?;
            while let Some(res_user_from) = cursor_from_user.try_next().await? {
                let mut cursor_from_user = collection_user
                    .find(bson::doc! { "_id": res_message.to }, None)
                    .await?;
                while let Some(res_user_to) = cursor_from_user.try_next().await? {
                    let username_json = MessageTwoUsers {
                        body: res_message.body.clone(),
                        to: res_user_to.username.clone(),
                        from: res_user_from.username.clone(),
                        time: res_message.time.clone(),
                    };

                    vec_messages.push(username_json);
                }
            }
        }
        Ok(vec_messages)
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
