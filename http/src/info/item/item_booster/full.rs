use super::HBoosterInfoPartial;
use crate::info::item::extended::HItemExtendedInfo;

#[derive(serde::Serialize)]
pub(crate) struct HBoosterInfoFull {
    #[serde(flatten)]
    partial_info: HBoosterInfoPartial,
    #[serde(flatten)]
    extended_info: HItemExtendedInfo,
}
impl From<&mut rc::BoosterMut<'_>> for HBoosterInfoFull {
    fn from(core_booster: &mut rc::BoosterMut) -> Self {
        Self {
            partial_info: core_booster.into(),
            extended_info: core_booster.into(),
        }
    }
}
