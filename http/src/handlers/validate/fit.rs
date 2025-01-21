use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use crate::{
    bridge::HBrError,
    cmd::HValidFitCmd,
    handlers::{get_guarded_sol, validate::HValidInfoParams, HGSolResult, HSingleErr},
    state::HAppState,
    util::HExecError,
};

pub(crate) async fn validate_fit(
    State(state): State<HAppState>,
    Path((sol_id, fit_id)): Path<(String, String)>,
    Query(params): Query<HValidInfoParams>,
    payload: Option<Json<HValidFitCmd>>,
) -> impl IntoResponse {
    let guarded_sol = match get_guarded_sol(&state.sol_mgr, &sol_id).await {
        HGSolResult::Sol(sol) => sol,
        HGSolResult::ErrResp(r) => return r,
    };
    let Json(payload) = payload.unwrap_or_default();
    let resp = match guarded_sol
        .lock()
        .await
        .validate_fit(&fit_id, payload, params.validation.into())
        .await
    {
        Ok(valid_info) => (StatusCode::OK, Json(valid_info)).into_response(),
        Err(br_err) => {
            let code = match &br_err {
                HBrError::FitIdCastFailed(_) => StatusCode::NOT_FOUND,
                HBrError::ExecFailed(exec_err) => match exec_err {
                    HExecError::FitNotFoundPrimary(_) => StatusCode::NOT_FOUND,
                    _ => StatusCode::INTERNAL_SERVER_ERROR,
                },
                _ => StatusCode::INTERNAL_SERVER_ERROR,
            };
            (code, Json(HSingleErr::from(br_err))).into_response()
        }
    };
    resp
}
