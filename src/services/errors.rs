use axum::{ response::{ IntoResponse, Response }, http::StatusCode, Json };
use serde_json::json;

#[derive(Clone)]
pub enum ApiError {
    OpenLibraryError(String),
    QueryParamMissing
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            Self::OpenLibraryError(msg) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                msg
            ),
            Self::QueryParamMissing => (
                StatusCode::BAD_REQUEST,
                "Query parameter 'query' is required".to_string()
            ),
        };

        (status, Json(json!({ "error": error_message }))).into_response()
    }
}
