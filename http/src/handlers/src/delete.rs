use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use crate::{handlers::SingleErr, state::AppState, util::ErrorKind};

pub(crate) async fn delete_source(State(state): State<AppState>, Path(alias): Path<String>) -> impl IntoResponse {
    match state.src_mgr.del(&alias).await {
        Ok(_) => StatusCode::NO_CONTENT.into_response(),
        Err(e) => {
            let code = match e.kind {
                ErrorKind::SrcNotFound(_) => StatusCode::NOT_FOUND,
                _ => StatusCode::INTERNAL_SERVER_ERROR,
            };
            (code, Json(SingleErr::from(e))).into_response()
        }
    }
}
