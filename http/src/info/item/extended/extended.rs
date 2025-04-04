use std::collections::HashMap;

use crate::shared::HEffectId;

use super::{HAttrVal, HEffect, HModificationInfo};

#[derive(serde::Serialize)]
pub(crate) struct HItemExtendedInfo {
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub(crate) attrs: HashMap<rc::AttrId, HAttrVal>,
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub(crate) effects: HashMap<HEffectId, HEffect>,
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub(crate) mods: HashMap<rc::AttrId, Vec<HModificationInfo>>,
}
impl HItemExtendedInfo {
    pub(in crate::info::item) fn mk_info(core_sol: &mut rc::SolarSystem, item_id: &rc::ItemId) -> Self {
        let attrs = match core_sol.iter_item_attrs(item_id) {
            Ok(core_attrs) => core_attrs.into_iter().map(|(k, v)| (k, HAttrVal::from(&v))).collect(),
            _ => HashMap::new(),
        };
        let effects = match core_sol.iter_item_effects(item_id) {
            Ok(core_effects) => core_effects
                .into_iter()
                .map(|(k, v)| (k.into(), HEffect::from(&v)))
                .collect(),
            _ => HashMap::new(),
        };
        let mods = match core_sol.iter_item_modifiers(item_id) {
            Ok(core_mods) => core_mods
                .into_iter()
                .map(|(k, v)| (k, v.into_iter().map(|m| HModificationInfo::from(&m)).collect()))
                .collect(),
            _ => HashMap::new(),
        };
        Self { attrs, effects, mods }
    }
}
