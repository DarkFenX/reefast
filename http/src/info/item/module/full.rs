use std::collections::HashMap;

use crate::info::{HAttrVal, HEffect, HItemInfoMode, HModificationInfo};

use super::HModuleInfoPartial;

#[derive(serde::Serialize)]
pub(crate) struct HModuleInfoFull {
    #[serde(flatten)]
    pub(crate) partial_info: HModuleInfoPartial,
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub(crate) attrs: HashMap<rc::EAttrId, HAttrVal>,
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub(crate) effects: HashMap<rc::EEffectId, HEffect>,
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub(crate) mods: HashMap<rc::EAttrId, Vec<HModificationInfo>>,
}
impl HModuleInfoFull {
    pub(super) fn mk_info(
        core_sol: &mut rc::SolarSystem,
        core_module_info: &rc::SolModuleInfo,
        item_mode: HItemInfoMode,
    ) -> Self {
        let partial_info = HModuleInfoPartial::mk_info(core_sol, core_module_info, item_mode);
        let attrs = match core_sol.get_item_attrs(&partial_info.id) {
            Ok(core_attrs) => core_attrs.into_iter().map(|(k, v)| (k, HAttrVal::from(&v))).collect(),
            _ => HashMap::new(),
        };
        let effects = match core_sol.iter_item_effects(&partial_info.id) {
            Ok(core_effects) => core_effects.into_iter().map(|(k, v)| (k, HEffect::from(&v))).collect(),
            _ => HashMap::new(),
        };
        let mods = match core_sol.iter_item_modifiers(&partial_info.id) {
            Ok(core_mods) => core_mods
                .into_iter()
                .map(|(k, v)| (k, v.into_iter().map(|m| HModificationInfo::from(&m)).collect()))
                .collect(),
            _ => HashMap::new(),
        };
        Self {
            partial_info,
            attrs,
            effects,
            mods,
        }
    }
}
