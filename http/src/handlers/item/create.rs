use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use crate::{
    cmd::HAddItemCommand,
    handlers::{get_guarded_sol, item::HItemInfoParams, HGSolResult, HSingleErr},
    state::HAppState,
};

pub(crate) async fn create_item(
    State(state): State<HAppState>,
    Path(sol_id): Path<String>,
    Query(params): Query<HItemInfoParams>,
    Json(payload): Json<HAddItemCommand>,
) -> impl IntoResponse {
    let guarded_sol = match get_guarded_sol(&state.sol_mgr, &sol_id).await {
        HGSolResult::Sol(sol) => sol,
        HGSolResult::ErrResp(r) => return r,
    };
    let resp = match guarded_sol.lock().await.add_item(payload, params.item.into()).await {
        Ok(item_info) => (StatusCode::CREATED, Json(item_info)).into_response(),
        Err(br_err) => (StatusCode::INTERNAL_SERVER_ERROR, Json(HSingleErr::from(br_err))).into_response(),
    };
    resp
}
