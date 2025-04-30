use chrono::{DateTime, Utc};
use mongodb::{
    bson::{doc, oid::ObjectId},
    error::Error,
    Database,
};
use serde::{Deserialize, Serialize};

const COLLECTION_NAME: &str = "users";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum UserRole {
    /// Root user of entire application
    Root,
    /// Other all users
    User,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub name: String,
    pub password: String,
    pub role: UserRole,
    #[serde(
        rename = "createdAt",
        with = "bson::serde_helpers::chrono_datetime_as_bson_datetime"
    )]
    pub created_at: DateTime<Utc>,
    #[serde(
        rename = "updatedAt",
        with = "bson::serde_helpers::chrono_datetime_as_bson_datetime"
    )]
    pub updated_at: DateTime<Utc>,
}

impl User {
    pub async fn find_by_name(db: &Database, name: &str) -> Result<Option<Self>, Error> {
        let collection = db.collection(COLLECTION_NAME);
        let user = collection.find_one(doc! {"name": name}).await?;

        Ok(user)
    }

    pub async fn create(db: &Database, user: Self) -> Result<Option<Self>, Error> {
        let collection = db.collection(COLLECTION_NAME);
        let result = collection.insert_one(user.clone()).await?;
        let user = collection
            .find_one(doc! {"_id": result.inserted_id})
            .await?;

        Ok(user)
    }
}
