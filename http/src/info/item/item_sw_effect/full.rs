use super::HSwEffectInfoPartial;
use crate::info::item::extended::HItemExtendedInfo;

#[derive(serde::Serialize)]
pub(crate) struct HSwEffectInfoFull {
    #[serde(flatten)]
    partial_info: HSwEffectInfoPartial,
    #[serde(flatten)]
    extended_info: HItemExtendedInfo,
}
impl From<&mut rc::SwEffectMut<'_>> for HSwEffectInfoFull {
    fn from(core_sw_effect: &mut rc::SwEffectMut) -> Self {
        Self {
            partial_info: core_sw_effect.into(),
            extended_info: core_sw_effect.into(),
        }
    }
}
