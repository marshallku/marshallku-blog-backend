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
        default = "default_date",
        with = "bson::serde_helpers::chrono_datetime_as_bson_datetime"
    )]
    pub created_at: DateTime<Utc>,

    /// Last update timestamp, automatically managed
    #[serde(
        rename = "updatedAt",
        default = "default_date",
        with = "bson::serde_helpers::chrono_datetime_as_bson_datetime"
    )]
    pub updated_at: DateTime<Utc>,

    /// Replies to this comment
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replies: Option<Vec<Self>>,
}

fn default_date() -> DateTime<Utc> {
    Utc::now()
}

/// Response model for backward compatibility
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CommentResponse {
    #[serde(rename = "_id")]
    pub id: String,

    pub name: String,

    #[serde(rename = "postSlug")]
    pub post_slug: String,

    #[serde(rename = "byPostAuthor")]
    pub by_post_author: bool,

    pub email: String,

    pub url: String,

    pub body: String,

    #[serde(rename = "parentCommentId")]
    pub parent_comment_id: Option<String>,

    #[serde(rename = "createdAt")]
    pub created_at: String,

    #[serde(rename = "updatedAt")]
    pub updated_at: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub replies: Option<Vec<CommentResponse>>,
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

    pub fn to_response(&self) -> CommentResponse {
        CommentResponse {
            id: self.id.unwrap().to_string(),
            name: self.name.clone(),
            post_slug: self.post_slug.clone(),
            by_post_author: self.by_post_author,
            email: self.email.clone(),
            url: self.url.clone(),
            body: self.body.clone(),
            parent_comment_id: self.parent_comment_id.clone().map(|id| id.to_string()),
            created_at: self.created_at.to_rfc3339(),
            updated_at: Some(self.updated_at.to_rfc3339()),
            replies: None,
        }
    }

    pub async fn get_by_slug(db: &Database, slug: &str) -> Result<Vec<CommentResponse>, Error> {
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

        let mut root_comments: Vec<CommentResponse> = Vec::new();
        let mut replies: Vec<Self> = Vec::new();

        for comment in all_comments {
            if comment.parent_comment_id.is_some()
                && comment.parent_comment_id.unwrap() != ObjectId::default()
            {
                replies.push(comment);
            } else {
                root_comments.push(comment.to_response());
            }
        }

        for comment in &mut root_comments {
            let mut replies_for_comment: Vec<Self> = Vec::new();

            for reply in replies.iter().rev() {
                if reply.parent_comment_id.is_some()
                    && reply.parent_comment_id.unwrap().to_string() == comment.id
                {
                    replies_for_comment.push(reply.clone());
                }
            }

            comment.replies = Some(
                replies_for_comment
                    .into_iter()
                    .map(|r| r.to_response())
                    .collect(),
            );
        }

        Ok(root_comments)
    }

    pub async fn get_recent(db: &Database, limit: i64) -> Result<Vec<CommentResponse>, Error> {
        let collection = db.collection::<Self>(COLLECTION_NAME);
        let mut cursor = collection
            .find(doc! {})
            .limit(limit)
            .sort(doc! {"createdAt": -1})
            .await?;
        let mut all_comments: Vec<CommentResponse> = Vec::new();

        while let Some(comment) = cursor.try_next().await? {
            all_comments.push(comment.to_response());
        }

        Ok(all_comments)
    }

    pub async fn delete(db: &Database, id: &str) -> Result<(), Error> {
        let collection = db.collection::<Self>(COLLECTION_NAME);
        let object_id = ObjectId::parse_str(id).map_err(|_| Error::custom("Invalid comment ID"))?;

        let result = collection.delete_one(doc! {"_id": object_id}).await?;

        if result.deleted_count == 0 {
            return Err(Error::custom("Comment not found"));
        }

        Ok(())
    }
}
