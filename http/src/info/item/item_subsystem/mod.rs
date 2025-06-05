use full::HSubsystemInfoFull;
use id::HSubsystemInfoId;
use partial::HSubsystemInfoPartial;

use crate::info::HItemInfoMode;

mod full;
mod id;
mod partial;

#[derive(serde::Serialize)]
#[serde(untagged)]
pub(crate) enum HSubsystemInfo {
    Id(HSubsystemInfoId),
    Partial(HSubsystemInfoPartial),
    Full(HSubsystemInfoFull),
}
impl HSubsystemInfo {
    pub(crate) fn mk_info(core_subsystem: &mut rc::SubsystemMut, item_mode: HItemInfoMode) -> Self {
        match item_mode {
            HItemInfoMode::Id => Self::Id(core_subsystem.into()),
            HItemInfoMode::Partial => Self::Partial(core_subsystem.into()),
            HItemInfoMode::Full => Self::Full(core_subsystem.into()),
        }
    }
}
