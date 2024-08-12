use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use crate::{
    bridge::HBrError,
    handlers::{fit::HFitInfoParams, get_guarded_sol, HGSolResult, HSingleErr},
    state::HAppState,
    util::HExecError,
};

pub(crate) async fn create_fit(
    State(state): State<HAppState>,
    Path(sol_id): Path<String>,
    Query(params): Query<HFitInfoParams>,
) -> impl IntoResponse {
    let guarded_sol = match get_guarded_sol(&state.sol_mgr, &sol_id).await {
        HGSolResult::Sol(sol) => sol,
        HGSolResult::ErrResp(r) => return r,
    };
    let resp = match guarded_sol
        .lock()
        .await
        .add_fit(params.fit.into(), params.item.into())
        .await
    {
        Ok(fit_info) => (StatusCode::CREATED, Json(fit_info)).into_response(),
        Err(br_err) => {
            let code = match &br_err {
                HBrError::ExecFailed(exec_err) => match exec_err {
                    HExecError::FitCapacityReached(_) => StatusCode::SERVICE_UNAVAILABLE,
                    _ => StatusCode::INTERNAL_SERVER_ERROR,
                },
                _ => StatusCode::INTERNAL_SERVER_ERROR,
            };
            (code, Json(HSingleErr::from(br_err))).into_response()
        }
    };
    resp
}
