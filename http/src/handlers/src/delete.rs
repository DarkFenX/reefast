use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use crate::{bridge::HBrErrorKind, handlers::HSingleErr, state::HAppState};

pub(crate) async fn delete_source(State(state): State<HAppState>, Path(alias): Path<String>) -> impl IntoResponse {
    let resp = match state.src_mgr.del(&alias).await {
        Ok(_) => StatusCode::NO_CONTENT.into_response(),
        Err(e) => {
            let code = match e.kind {
                HBrErrorKind::SrcNotFound(_) => StatusCode::NOT_FOUND,
                _ => StatusCode::INTERNAL_SERVER_ERROR,
            };
            (code, Json(HSingleErr::from(e))).into_response()
        }
    };
    resp
}
