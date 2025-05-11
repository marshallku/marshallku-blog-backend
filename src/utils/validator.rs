use axum::{
    async_trait,
    extract::{rejection::JsonRejection, FromRequest, Json, Request},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde_json::json;
use validator::Validate;

pub struct ValidatedJson<T>(pub T);

#[async_trait]
impl<T, S> FromRequest<S> for ValidatedJson<T>
where
    T: Validate + serde::de::DeserializeOwned,
    S: Send + Sync,
    Json<T>: FromRequest<S, Rejection = JsonRejection>,
{
    type Rejection = Response;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let Json(payload) = Json::<T>::from_request(req, state).await.map_err(|err| {
            let message = format!("Invalid JSON: {}", err);
            let json_error = json!({
                "error": {
                    "message": message,
                    "type": "json_extraction_error"
                }
            });
            (StatusCode::BAD_REQUEST, axum::Json(json_error)).into_response()
        })?;

        if let Err(validation_errors) = payload.validate() {
            let json_error = json!({
                "error": {
                    "message": "Validation failed",
                    "type": "validation_error",
                    "details": validation_errors
                }
            });
            return Err((StatusCode::BAD_REQUEST, axum::Json(json_error)).into_response());
        }

        Ok(ValidatedJson(payload))
    }
}
