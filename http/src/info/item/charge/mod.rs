use full::HChargeInfoFull;
use id::HChargeInfoId;
use partial::HChargeInfoPartial;

use crate::info::HItemInfoMode;

mod full;
mod id;
mod partial;

#[derive(serde::Serialize)]
#[serde(untagged)]
pub(crate) enum HChargeInfo {
    Id(HChargeInfoId),
    Partial(HChargeInfoPartial),
    Full(HChargeInfoFull),
}
impl HChargeInfo {
    pub(crate) fn mk_info(core_charge: &mut rc::ChargeMut, item_mode: HItemInfoMode) -> Self {
        match item_mode {
            HItemInfoMode::Id => Self::Id(core_charge.into()),
            HItemInfoMode::Partial => Self::Partial(core_charge.into()),
            HItemInfoMode::Full => Self::Full(core_charge.into()),
        }
    }
}
