use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};

use crate::{state::AppState, util::ErrorKind};

pub(crate) async fn delete_source(State(state): State<Arc<AppState>>, Path(alias): Path<String>) -> impl IntoResponse {
    match state.src_mgr.del(&alias).await {
        Ok(_) => StatusCode::NO_CONTENT,
        Err(e) if matches!(e.kind, ErrorKind::SrcNotFound(_)) => StatusCode::NOT_FOUND,
        _ => StatusCode::INTERNAL_SERVER_ERROR,
    }
}
