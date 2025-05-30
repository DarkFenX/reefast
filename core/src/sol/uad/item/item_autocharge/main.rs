use crate::{
    ad,
    sol::{
        FitKey, ItemId, ItemKey,
        uad::item::{EffectModes, Projs, UadItemBase},
    },
    src::Src,
    util::{Named, RMap},
};

#[derive(Clone)]
pub(in crate::sol) struct UadAutocharge {
    base: UadItemBase,
    fit_key: FitKey,
    cont_item_key: ItemKey,
    cont_a_effect_id: ad::AEffectId,
    projs: Projs,
    force_disable: bool,
}
impl UadAutocharge {
    pub(in crate::sol) fn new(
        src: &Src,
        item_id: ItemId,
        a_item_id: ad::AItemId,
        fit_key: FitKey,
        cont_item_key: ItemKey,
        cont_a_effect_id: ad::AEffectId,
        cont_a_state: ad::AState,
        force_disable: bool,
    ) -> Self {
        Self {
            base: UadItemBase::new(src, item_id, a_item_id, cont_a_state),
            fit_key,
            cont_item_key,
            cont_a_effect_id,
            projs: Projs::new(),
            force_disable,
        }
    }
    // Item base methods
    pub(in crate::sol) fn get_item_id(&self) -> ItemId {
        self.base.get_item_id()
    }
    pub(in crate::sol) fn get_a_item_id(&self) -> ad::AItemId {
        self.base.get_a_item_id()
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
        match self.force_disable {
            true => ad::AState::Ghost,
            false => self.base.get_a_state(),
        }
    }
    pub(in crate::sol) fn set_a_state(&mut self, state: ad::AState) {
        self.base.set_a_state(state)
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
    pub(in crate::sol::uad::item) fn update_a_data(&mut self, _src: &Src) {
        // Just panic to expose attempts to reload it, since autocharges should never be reloaded.
        // Instead, they are removed and re-added when parent item changes.
        unreachable!("autocharges shouldn't be reloaded");
    }
    // Item-specific methods
    pub(in crate::sol) fn get_force_disable(&self) -> bool {
        self.force_disable
    }
    pub(in crate::sol) fn set_force_disable(&mut self, force_disable: bool) {
        self.force_disable = force_disable
    }
    pub(in crate::sol) fn get_fit_key(&self) -> FitKey {
        self.fit_key
    }
    pub(in crate::sol) fn get_cont_item_key(&self) -> ItemKey {
        self.cont_item_key
    }
    pub(in crate::sol) fn get_cont_effect_id(&self) -> ad::AEffectId {
        self.cont_a_effect_id
    }
    pub(in crate::sol) fn get_projs(&self) -> &Projs {
        &self.projs
    }
    pub(in crate::sol) fn get_projs_mut(&mut self) -> &mut Projs {
        &mut self.projs
    }
}
impl Named for UadAutocharge {
    fn get_name() -> &'static str {
        "Autocharge"
    }
}
impl std::fmt::Display for UadAutocharge {
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
