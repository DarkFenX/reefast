use full::HFitInfoFull;
use id::HFitInfoId;

use crate::info::{HFitInfoMode, HItemInfoMode};

mod full;
mod id;

#[derive(serde::Serialize)]
#[serde(untagged)]
pub(crate) enum HFitInfo {
    Id(HFitInfoId),
    Full(HFitInfoFull),
}
impl HFitInfo {
    pub(crate) fn mk_info(
        core_ss: &mut rc::SolarSystem,
        fit_id: &rc::ReeId,
        fit_mode: HFitInfoMode,
        item_mode: HItemInfoMode,
    ) -> Self {
        match fit_mode {
            HFitInfoMode::Id => Self::Id(fit_id.into()),
            HFitInfoMode::Full => Self::Full(HFitInfoFull::mk_info(core_ss, fit_id, item_mode)),
        }
    }
}
