use axum::{
    Json,
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
};

use crate::{
    bridge::HBrError,
    cmd::HAddSolCmd,
    handlers::{HSingleErr, sol::HSolInfoParams},
    state::HAppState,
    util::HExecError,
};

#[derive(Default, serde::Deserialize)]
pub(crate) struct HCreateSolReq {
    src_alias: Option<String>,
    #[serde(flatten)]
    cmd: HAddSolCmd,
}

pub(crate) async fn create_sol(
    State(state): State<HAppState>,
    Query(params): Query<HSolInfoParams>,
    payload: Option<Json<HCreateSolReq>>,
) -> impl IntoResponse {
    let Json(payload) = payload.unwrap_or_default();
    let src = match state.src_mgr.get(payload.src_alias.as_deref()).await {
        Ok(src) => src,
        Err(br_err) => {
            let code = match br_err {
                HBrError::SrcNotFound(_) => StatusCode::UNPROCESSABLE_ENTITY,
                HBrError::NoDefaultSrc => StatusCode::UNPROCESSABLE_ENTITY,
                _ => StatusCode::INTERNAL_SERVER_ERROR,
            };
            return (code, Json(HSingleErr::from(br_err))).into_response();
        }
    };
    let sol_info = match state
        .sol_mgr
        .add_sol(
            payload.cmd,
            src,
            params.sol.unwrap_or_default(),
            params.fleet.unwrap_or_default(),
            params.fit.unwrap_or_default(),
            params.item.unwrap_or_default(),
        )
        .await
    {
        Ok(sol_info) => sol_info,
        Err(br_err) => {
            let code = match &br_err {
                HBrError::ExecFailed(
                    HExecError::InvalidDpsProfileEm(_)
                    | HExecError::InvalidDpsProfileTherm(_)
                    | HExecError::InvalidDpsProfileKin(_)
                    | HExecError::InvalidDpsProfileExpl(_)
                ) => StatusCode::BAD_REQUEST,
                _ => StatusCode::INTERNAL_SERVER_ERROR,
            };
            return (code, Json(HSingleErr::from(br_err))).into_response();
        }
    };
    (StatusCode::CREATED, Json(sol_info)).into_response()
}
