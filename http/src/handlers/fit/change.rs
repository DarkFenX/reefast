use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use crate::{
    cmd::{CmdResp, FitCommand},
    handlers::{fit::FitInfoParams, get_guarded_ss, GSsResult, SingleErr},
    info::FitInfo,
    state::AppState,
};

#[derive(serde::Deserialize)]
pub(crate) struct FitChangeReq {
    commands: Vec<FitCommand>,
}

#[derive(serde::Serialize)]
pub(crate) struct FitChangeResp {
    fit: FitInfo,
    cmd_results: Vec<CmdResp>,
}
impl FitChangeResp {
    pub(crate) fn new(fit: FitInfo, cmd_results: Vec<CmdResp>) -> Self {
        Self { fit, cmd_results }
    }
}

pub(crate) async fn change_fit(
    State(state): State<AppState>,
    Path((ssid, fid)): Path<(String, String)>,
    Query(params): Query<FitInfoParams>,
    Json(payload): Json<FitChangeReq>,
) -> impl IntoResponse {
    let guarded_ss = match get_guarded_ss(&state.ss_mgr, &ssid).await {
        GSsResult::SolSys(ss) => ss,
        GSsResult::ErrResp(r) => return r,
    };
    let resp = match guarded_ss
        .lock()
        .await
        .execute_fit_commands(&fid, payload.commands, params.fit.into(), params.item.into())
        .await
    {
        Ok((fit_info, cmd_results)) => {
            let resp = FitChangeResp::new(fit_info, cmd_results);
            (StatusCode::OK, Json(resp)).into_response()
        }
        Err(e) => {
            let code = StatusCode::INTERNAL_SERVER_ERROR;
            (code, Json(SingleErr::from(e))).into_response()
        }
    };
    resp
}
