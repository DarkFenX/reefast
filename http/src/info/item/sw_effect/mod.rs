use full::HSwEffectInfoFull;
use id::HSwEffectInfoId;
use partial::HSwEffectInfoPartial;

use crate::info::HItemInfoMode;

mod full;
mod id;
mod partial;

#[derive(serde::Serialize)]
#[serde(untagged)]
pub(crate) enum HSwEffectInfo {
    Id(HSwEffectInfoId),
    Partial(HSwEffectInfoPartial),
    Full(HSwEffectInfoFull),
}
impl HSwEffectInfo {
    pub(crate) fn mk_info(core_sw_effect: &mut rc::SwEffectMut, item_mode: HItemInfoMode) -> Self {
        match item_mode {
            HItemInfoMode::Id => Self::Id(core_sw_effect.into()),
            HItemInfoMode::Partial => Self::Partial(core_sw_effect.into()),
            HItemInfoMode::Full => Self::Full(core_sw_effect.into()),
        }
    }
}
