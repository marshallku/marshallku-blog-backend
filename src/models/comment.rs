use bson::doc;
use chrono::{DateTime, Utc};
use mongodb::{bson::oid::ObjectId, error::Error, Database};
use serde::{Deserialize, Serialize};

const COLLECTION_NAME: &str = "comment";

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Comment {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,

    /// Name of the comment author, defaults to `익명`
    #[serde(default = "default_name")]
    pub name: String,

    /// Slug of the post this comment belongs to (indexed field)
    #[serde(rename = "postSlug")]
    pub post_slug: String,

    /// Whether this comment was made by the post author
    #[serde(rename = "byPostAuthor", default)]
    pub by_post_author: bool,

    /// Optional password for comment editing/deletion
    #[serde(default)]
    pub password: String,

    /// Optional email of the commenter
    #[serde(default)]
    pub email: String,

    /// Optional website URL of the commenter
    #[serde(default)]
    pub url: String,

    /// The comment content
    pub body: String,

    /// Reference to parent comment if this is a reply
    #[serde(rename = "parentCommentId", skip_serializing_if = "Option::is_none")]
    pub parent_comment_id: Option<ObjectId>,

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

fn default_name() -> String {
    "익명".to_string()
}

impl Comment {
    #[allow(dead_code)]
    pub async fn create(db: &Database, comment: Self) -> Result<Self, Error> {
        let collection = db.collection(COLLECTION_NAME);
        let result = collection.insert_one(comment.clone()).await?;
        let comment = collection
            .find_one(doc! {"_id": result.inserted_id})
            .await?;

        if comment.is_none() {
            return Err(Error::custom("Failed to create comment"));
        }

        Ok(comment.unwrap())
    }
}
