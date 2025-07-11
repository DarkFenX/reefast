use crate::{
    ad,
    def::{FitKey, ItemId},
    src::Src,
    uad::item::{EffectModes, UadItemBase, bool_to_state_offline, state_to_bool},
    util::{Named, RMap},
};

#[derive(Clone)]
pub(crate) struct UadSubsystem {
    base: UadItemBase,
    fit_key: FitKey,
}
impl UadSubsystem {
    pub(crate) fn new(src: &Src, item_id: ItemId, a_item_id: ad::AItemId, fit_key: FitKey, state: bool) -> Self {
        Self {
            base: UadItemBase::new(src, item_id, a_item_id, bool_to_state_offline(state)),
            fit_key,
        }
    }
    // Item base methods
    pub(crate) fn get_item_id(&self) -> ItemId {
        self.base.get_item_id()
    }
    pub(crate) fn get_a_item_id(&self) -> ad::AItemId {
        self.base.get_a_item_id()
    }
    pub(crate) fn set_a_item_id(&mut self, src: &Src, a_item_id: ad::AItemId) {
        self.base.set_a_item_id_and_reload(src, a_item_id);
    }
    pub(crate) fn get_a_group_id(&self) -> Option<ad::AItemGrpId> {
        self.base.get_a_group_id()
    }
    pub(crate) fn get_a_category_id(&self) -> Option<ad::AItemCatId> {
        self.base.get_a_category_id()
    }
    pub(crate) fn get_a_attrs(&self) -> Option<&RMap<ad::AAttrId, ad::AAttrVal>> {
        self.base.get_a_attrs()
    }
    pub(crate) fn get_a_effect_datas(&self) -> Option<&RMap<ad::AEffectId, ad::AItemEffectData>> {
        self.base.get_a_effect_datas()
    }
    pub(crate) fn get_a_defeff_id(&self) -> Option<Option<ad::AEffectId>> {
        self.base.get_a_defeff_id()
    }
    pub(crate) fn get_a_skill_reqs(&self) -> Option<&RMap<ad::AItemId, ad::ASkillLevel>> {
        self.base.get_a_skill_reqs()
    }
    pub(crate) fn get_a_xt(&self) -> Option<&ad::AItemXt> {
        self.base.get_a_xt()
    }
    pub(crate) fn get_a_state(&self) -> ad::AState {
        self.base.get_a_state()
    }
    pub(crate) fn get_effect_modes(&self) -> &EffectModes {
        self.base.get_effect_modes()
    }
    pub(crate) fn get_effect_modes_mut(&mut self) -> &mut EffectModes {
        self.base.get_effect_modes_mut()
    }
    pub(crate) fn is_loaded(&self) -> bool {
        self.base.is_loaded()
    }
    pub(in crate::uad::item) fn update_a_data(&mut self, src: &Src) {
        self.base.update_a_data(src);
    }
    // Item-specific methods
    pub(crate) fn get_subsystem_state(&self) -> bool {
        state_to_bool(self.base.get_a_state())
    }
    pub(crate) fn set_subsystem_state(&mut self, state: bool) {
        self.base.set_a_state(bool_to_state_offline(state))
    }
    pub(crate) fn get_fit_key(&self) -> FitKey {
        self.fit_key
    }
    pub(crate) fn get_a_slot(&self) -> Option<ad::ASlotIndex> {
        match self.get_a_xt() {
            Some(a_xt) => a_xt.subsystem_slot,
            None => None,
        }
    }
}
impl Named for UadSubsystem {
    fn get_name() -> &'static str {
        "Subsystem"
    }
}
impl std::fmt::Display for UadSubsystem {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}(item_id={}, a_item_id={})",
            Self::get_name(),
            self.get_item_id(),
            self.get_a_item_id(),
        )
    }
}
