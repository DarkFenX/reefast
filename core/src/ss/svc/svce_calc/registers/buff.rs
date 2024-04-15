use std::collections::HashSet;

use crate::{
    ad,
    defs::{EEffectId, SsItemId},
};

// Intended to hold data about buffs which use on-item attributes which define buff type
pub(in crate::ss::svc::svce_calc) struct BuffRegister {
    effects: HashSet<(SsItemId, EEffectId)>,
}
impl BuffRegister {
    pub(in crate::ss::svc::svce_calc) fn new() -> Self {
        Self {
            effects: HashSet::new(),
        }
    }
    // Modification methods
    pub(in crate::ss::svc::svce_calc) fn reg_effect(&mut self, item_id: SsItemId, effect: &ad::AEffect) {
        if uses_default_attrs(effect) {
            self.effects.insert((item_id, effect.id));
        }
    }
    pub(in crate::ss::svc::svce_calc) fn unreg_effect(&mut self, item_id: SsItemId, effect: &ad::AEffect) {
        if uses_default_attrs(effect) {
            self.effects.remove(&(item_id, effect.id));
        }
    }
}

fn uses_default_attrs(effect: &ad::AEffect) -> bool {
    match &effect.buff {
        Some(buff_info) => match buff_info.data_source {
            ad::AEffectBuffDataSrc::DefaultAttrs => true,
            _ => false,
        },
        _ => false,
    }
}
