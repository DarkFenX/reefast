use full::HSsInfoFull;
use id::HSsInfoId;

use crate::info::{HFitInfoMode, HItemInfoMode, HSsInfoMode};

mod full;
mod id;

#[derive(serde::Serialize)]
#[serde(untagged)]
pub(crate) enum HSsInfo {
    Id(HSsInfoId),
    Full(HSsInfoFull),
}
impl HSsInfo {
    pub(crate) fn mk_info(
        ss_id: String,
        core_ss: &mut rc::SolarSystem,
        ss_mode: HSsInfoMode,
        fit_mode: HFitInfoMode,
        item_mode: HItemInfoMode,
    ) -> Self {
        match ss_mode {
            HSsInfoMode::Id => Self::Id(ss_id.into()),
            HSsInfoMode::Full => Self::Full(HSsInfoFull::mk_info(ss_id, core_ss, fit_mode, item_mode)),
        }
    }
}
