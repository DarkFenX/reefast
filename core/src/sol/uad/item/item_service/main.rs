use crate::{
    ad,
    sol::{
        FitKey, ItemId, ServiceState,
        uad::item::{EffectModes, UadItemBase},
    },
    src::Src,
    util::{Named, RMap},
};

#[derive(Clone)]
pub(in crate::sol) struct UadService {
    base: UadItemBase,
    fit_key: FitKey,
}
impl UadService {
    pub(in crate::sol) fn new(
        src: &Src,
        item_id: ItemId,
        a_item_id: ad::AItemId,
        fit_key: FitKey,
        state: ServiceState,
    ) -> Self {
        Self {
            base: UadItemBase::new(src, item_id, a_item_id, state.into()),
            fit_key,
        }
    }
    // Item base methods
    pub(in crate::sol) fn get_item_id(&self) -> ItemId {
        self.base.get_item_id()
    }
    pub(in crate::sol) fn get_a_item_id(&self) -> ad::AItemId {
        self.base.get_a_item_id()
    }
    pub(in crate::sol) fn set_a_item_id(&mut self, src: &Src, a_item_id: ad::AItemId) {
        self.base.set_a_item_id_and_reload(src, a_item_id);
    }
    pub(in crate::sol) fn get_a_group_id(&self) -> Option<ad::AItemGrpId> {
        self.base.get_a_group_id()
    }
    pub(in crate::sol) fn get_a_category_id(&self) -> Option<ad::AItemCatId> {
        self.base.get_a_category_id()
    }
    pub(in crate::sol) fn get_a_attrs(&self) -> Option<&RMap<ad::AAttrId, ad::AAttrVal>> {
        self.base.get_a_attrs()
    }
    pub(in crate::sol) fn get_a_effect_datas(&self) -> Option<&RMap<ad::AEffectId, ad::AItemEffectData>> {
        self.base.get_a_effect_datas()
    }
    pub(in crate::sol) fn get_a_defeff_id(&self) -> Option<Option<ad::AEffectId>> {
        self.base.get_a_defeff_id()
    }
    pub(in crate::sol) fn get_a_skill_reqs(&self) -> Option<&RMap<ad::AItemId, ad::ASkillLevel>> {
        self.base.get_a_skill_reqs()
    }
    pub(in crate::sol) fn get_a_extras(&self) -> Option<&ad::AItemExtras> {
        self.base.get_a_extras()
    }
    pub(in crate::sol) fn get_a_state(&self) -> ad::AState {
        self.base.get_a_state()
    }
    pub(in crate::sol) fn get_effect_modes(&self) -> &EffectModes {
        self.base.get_effect_modes()
    }
    pub(in crate::sol) fn get_effect_modes_mut(&mut self) -> &mut EffectModes {
        self.base.get_effect_modes_mut()
    }
    pub(in crate::sol) fn is_loaded(&self) -> bool {
        self.base.is_loaded()
    }
    pub(in crate::sol::uad::item) fn update_a_data(&mut self, src: &Src) {
        self.base.update_a_data(src);
    }
    // Item-specific methods
    pub(in crate::sol) fn get_service_state(&self) -> ServiceState {
        self.base.get_a_state().into()
    }
    pub(in crate::sol) fn set_service_state(&mut self, state: ServiceState) {
        self.base.set_a_state(state.into())
    }
    pub(in crate::sol) fn get_fit_key(&self) -> FitKey {
        self.fit_key
    }
}
impl Named for UadService {
    fn get_name() -> &'static str {
        "Service"
    }
}
impl std::fmt::Display for UadService {
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
