#[cfg(test)]
mod tests {

    use axum::{
        body::Body,
        http::{Request, StatusCode},
        routing::get,
        Router,
    };
    use tower::ServiceExt;

    use crate::{controllers::health, env::state::AppState};

    #[tokio::test]
    async fn should_response() {
        let app: Router<AppState> = Router::new().route("/", get(health::get));
        let state = AppState::new().await.unwrap();
        let response = app
            .with_state(state)
            .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }
}
