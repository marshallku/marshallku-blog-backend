use std::fmt::Display;

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

impl Display for UserRole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UserRole::Root => write!(f, "Root"),
            UserRole::User => write!(f, "User"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    /// User id
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    /// User name
    pub name: String,
    #[serde(skip_serializing)]
    /// User password
    pub password: String,
    /// Role of the user
    ///
    /// - `Root`: Root user of entire application
    /// - `User`: Other all users
    pub role: UserRole,
    /// Creation timestamp, automatically managed
    #[serde(
        rename = "createdAt",
        with = "bson::serde_helpers::chrono_datetime_as_bson_datetime"
    )]
    pub created_at: DateTime<Utc>,
    /// Last update timestamp, automatically managed
    #[serde(
        rename = "updatedAt",
        with = "bson::serde_helpers::chrono_datetime_as_bson_datetime"
    )]
    pub updated_at: DateTime<Utc>,
}

impl User {
    pub async fn create(db: &Database, user: Self) -> Result<Self, Error> {
        let collection = db.collection(COLLECTION_NAME);
        let result = collection.insert_one(user.clone()).await?;
        let user = collection
            .find_one(doc! {"_id": result.inserted_id})
            .await?;

        if user.is_none() {
            return Err(Error::custom("Failed to create user"));
        }

        Ok(user.unwrap())
    }

    pub async fn find_by_id(db: &Database, id: &str) -> Result<Self, Error> {
        let collection = db.collection(COLLECTION_NAME);
        let id = ObjectId::parse_str(id).map_err(|e| {
            log::error!("[User] Error parsing token id: {:?}", e);
            Error::custom("Invalid token id")
        })?;
        let user = collection.find_one(doc! {"_id": id}).await?;

        if user.is_none() {
            return Err(Error::custom("User not found"));
        }

        Ok(user.unwrap())
    }

    pub async fn find_by_name(db: &Database, name: &str) -> Result<Self, Error> {
        let collection = db.collection(COLLECTION_NAME);
        let user = collection.find_one(doc! {"name": name}).await?;

        if user.is_none() {
            return Err(Error::custom("User not found"));
        }

        Ok(user.unwrap())
    }
}
