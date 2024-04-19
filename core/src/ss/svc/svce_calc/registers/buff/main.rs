use crate::{
    ad,
    defs::{EAttrId, EEffectId, SsItemId},
    ss::svc::svce_calc::SsAttrMod,
    util::StMapSetL1,
};

// Intended to hold data about modifiers which originated from buffs defined using on-item attribute
pub(in crate::ss::svc::svce_calc) struct SsBuffRegister {
    pub(super) effects: StMapSetL1<SsItemId, EEffectId>,
    pub(super) modifiers: StMapSetL1<(SsItemId, EAttrId), SsAttrMod>,
}
impl SsBuffRegister {
    pub(in crate::ss::svc::svce_calc) fn new() -> Self {
        Self {
            effects: StMapSetL1::new(),
            modifiers: StMapSetL1::new(),
        }
    }
    // Effect methods
    pub(in crate::ss::svc::svce_calc) fn get_effects(
        &self,
        item_id: &SsItemId,
    ) -> impl ExactSizeIterator<Item = &EEffectId> {
        self.effects.get(&item_id)
    }
    pub(in crate::ss::svc::svce_calc) fn reg_effect(&mut self, item_id: SsItemId, effect: &ad::AEffect) {
        if uses_default_attrs(effect) {
            self.effects.add_entry(item_id, effect.id);
        }
    }
    pub(in crate::ss::svc::svce_calc) fn unreg_effect(&mut self, item_id: SsItemId, effect: &ad::AEffect) {
        if uses_default_attrs(effect) {
            self.effects.remove_entry(&item_id, &effect.id);
        }
    }
    // Modifier methods
    pub(in crate::ss::svc::svce_calc) fn extract_dependent_mods(
        &mut self,
        item_id: &SsItemId,
        buff_type_attr_id: &EAttrId,
    ) -> Option<impl ExactSizeIterator<Item = SsAttrMod>> {
        self.modifiers.remove_key(&(*item_id, *buff_type_attr_id))
    }
    pub(in crate::ss::svc::svce_calc) fn reg_dependent_mod(
        &mut self,
        item_id: SsItemId,
        buff_type_attr_id: EAttrId,
        ss_mod: SsAttrMod,
    ) {
        self.modifiers.add_entry((item_id, buff_type_attr_id), ss_mod)
    }
    pub(in crate::ss::svc::svce_calc) fn unreg_dependent_mod(
        &mut self,
        item_id: &SsItemId,
        buff_type_attr_id: &EAttrId,
        ss_mod: &SsAttrMod,
    ) {
        self.modifiers.remove_entry(&(*item_id, *buff_type_attr_id), &ss_mod)
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
