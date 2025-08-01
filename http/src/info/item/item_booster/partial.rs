use std::collections::HashMap;

use rc::{ItemCommon, Lender};

use crate::{info::item::item_booster::side_effect::HSideEffectInfo, shared::HEffectId};

#[serde_with::serde_as]
#[derive(serde::Serialize)]
pub(crate) struct HBoosterInfoPartial {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    id: rc::ItemId,
    kind: &'static str,
    type_id: rc::ItemTypeId,
    #[serde_as(as = "serde_with::DisplayFromStr")]
    fit_id: rc::FitId,
    #[serde(skip_serializing_if = "Option::is_none")]
    slot: Option<rc::SlotIndex>,
    enabled: bool,
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    side_effects: HashMap<HEffectId, HSideEffectInfo>,
}
impl From<&mut rc::BoosterMut<'_>> for HBoosterInfoPartial {
    fn from(core_booster: &mut rc::BoosterMut) -> Self {
        Self {
            id: core_booster.get_item_id(),
            kind: "booster",
            type_id: core_booster.get_type_id(),
            fit_id: core_booster.get_fit().get_fit_id(),
            slot: core_booster.get_slot(),
            enabled: core_booster.get_state(),
            side_effects: core_booster
                .iter_side_effects_mut()
                .map_into_iter(|core_side_effect| (core_side_effect.get_effect_id().into(), core_side_effect.into()))
                .collect(),
        }
    }
}
