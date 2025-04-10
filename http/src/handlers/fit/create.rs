use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
};

use crate::{
    cmd::HAddFitCmd,
    handlers::{HGSolResult, HSingleErr, fit::HFitInfoParams, get_guarded_sol},
    state::HAppState,
};

#[allow(clippy::let_and_return)]
pub(crate) async fn create_fit(
    State(state): State<HAppState>,
    Path(sol_id): Path<String>,
    Query(params): Query<HFitInfoParams>,
    payload: Option<Json<HAddFitCmd>>,
) -> impl IntoResponse {
    let guarded_sol = match get_guarded_sol(&state.sol_mgr, &sol_id).await {
        HGSolResult::Sol(sol) => sol,
        HGSolResult::ErrResp(r) => return r,
    };
    let Json(payload) = payload.unwrap_or_default();
    let resp = match guarded_sol
        .lock()
        .await
        .add_fit(
            &state.tpool,
            payload,
            params.fit.unwrap_or_default(),
            params.item.unwrap_or_default(),
        )
        .await
    {
        Ok(fit_info) => (StatusCode::CREATED, Json(fit_info)).into_response(),
        Err(br_err) => (StatusCode::INTERNAL_SERVER_ERROR, Json(HSingleErr::from(br_err))).into_response(),
    };
    resp
}
