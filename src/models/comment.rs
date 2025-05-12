use bson::doc;
use chrono::{DateTime, Utc};
use futures::TryStreamExt;
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

    /// Replies to this comment
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replies: Option<Vec<Self>>,
}

fn default_name() -> String {
    "익명".to_string()
}

impl Comment {
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

    pub async fn get_by_slug(db: &Database, slug: &str) -> Result<Vec<Self>, Error> {
        let collection = db.collection::<Self>(COLLECTION_NAME);

        log::info!("Getting comments for slug: {}", slug);

        let mut cursor = collection
            .find(doc! {"postSlug": slug})
            .sort(doc! {"createdAt": -1})
            .await?;
        let mut all_comments: Vec<Self> = Vec::new();
        while let Some(comment) = cursor.try_next().await? {
            all_comments.push(comment);
        }

        let mut root_comments: Vec<Self> = Vec::new();
        let mut replies: Vec<Self> = Vec::new();

        for comment in all_comments {
            if comment.parent_comment_id.is_some()
                && comment.parent_comment_id.unwrap() != ObjectId::default()
            {
                replies.push(comment);
            } else {
                root_comments.push(comment);
            }
        }

        for comment in &mut root_comments {
            let mut replies_for_comment: Vec<Self> = Vec::new();

            for reply in &replies {
                if reply.parent_comment_id.is_some()
                    && reply.parent_comment_id.unwrap() == comment.id.unwrap()
                {
                    replies_for_comment.push(reply.clone());
                }
            }

            comment.replies = Some(replies_for_comment);
        }

        Ok(root_comments)
    }
}
