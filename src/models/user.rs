use chrono::{DateTime, Utc};
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum UserRole {
    /// Root user of entire application
    Root,
    /// Other all users
    User,
}

#[derive(Debug, Serialize, Deserialize)]
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
