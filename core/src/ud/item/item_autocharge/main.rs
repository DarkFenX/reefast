use crate::{
    ad::{AAttrId, AAttrVal, AEffectId, AItemCatId, AItemEffectData, AItemGrpId, AItemId, ASkillLevel, AState},
    def::ItemId,
    misc::EffectMode,
    rd::{REffectKey, RItemAXt},
    src::Src,
    ud::{
        UFitKey, UItemKey,
        item::{Projs, UEffectUpdates, UItemBase},
    },
    util::{Named, RMap, RSet},
};

#[derive(Clone)]
pub(crate) struct UAutocharge {
    pub(super) base: UItemBase,
    fit_key: UFitKey,
    cont_item_key: UItemKey,
    cont_effect_key: REffectKey,
    projs: Projs,
    activated: bool,
    force_disabled: bool,
}
impl UAutocharge {
    pub(crate) fn new(
        item_id: ItemId,
        type_id: AItemId,
        fit_key: UFitKey,
        cont_item_key: UItemKey,
        cont_effect_key: REffectKey,
        activated: bool,
        force_disabled: bool,
        src: &Src,
        reuse_eupdates: &mut UEffectUpdates,
    ) -> Self {
        Self {
            base: UItemBase::new(
                item_id,
                type_id,
                get_state(activated, force_disabled),
                src,
                reuse_eupdates,
            ),
            fit_key,
            cont_item_key,
            cont_effect_key,
            projs: Projs::new(),
            activated,
            force_disabled,
        }
    }
    // Item base methods
    pub(crate) fn get_item_id(&self) -> ItemId {
        self.base.get_item_id()
    }
    pub(crate) fn get_type_id(&self) -> AItemId {
        self.base.get_type_id()
    }
    pub(crate) fn get_group_id(&self) -> Option<AItemGrpId> {
        self.base.get_group_id()
    }
    pub(crate) fn get_category_id(&self) -> Option<AItemCatId> {
        self.base.get_category_id()
    }
    pub(crate) fn get_attrs(&self) -> Option<&RMap<AAttrId, AAttrVal>> {
        self.base.get_attrs()
    }
    pub(crate) fn get_effect_datas(&self) -> Option<&RMap<REffectKey, AItemEffectData>> {
        self.base.get_effect_datas()
    }
    pub(crate) fn get_defeff_key(&self) -> Option<Option<REffectKey>> {
        self.base.get_defeff_key()
    }
    pub(crate) fn get_skill_reqs(&self) -> Option<&RMap<AItemId, ASkillLevel>> {
        self.base.get_skill_reqs()
    }
    pub(crate) fn get_axt(&self) -> Option<&RItemAXt> {
        self.base.get_axt()
    }
    pub(crate) fn get_state(&self) -> AState {
        self.base.get_state()
    }
    pub(crate) fn get_reffs(&self) -> Option<&RSet<REffectKey>> {
        self.base.get_reffs()
    }
    pub(in crate::ud::item) fn start_all_reffs(&self, reuse_eupdates: &mut UEffectUpdates, src: &Src) {
        self.base.start_all_reffs(reuse_eupdates, src);
    }
    pub(in crate::ud::item) fn stop_all_reffs(&self, reuse_eupdates: &mut UEffectUpdates, src: &Src) {
        self.base.stop_all_reffs(reuse_eupdates, src)
    }
    pub(in crate::ud::item) fn get_effect_key_mode(&self, effect_key: &REffectKey) -> EffectMode {
        self.base.get_effect_key_mode(effect_key)
    }
    pub(in crate::ud::item) fn set_effect_mode(
        &mut self,
        effect_id: AEffectId,
        effect_mode: EffectMode,
        reuse_eupdates: &mut UEffectUpdates,
        src: &Src,
    ) {
        self.base.set_effect_mode(effect_id, effect_mode, reuse_eupdates, src)
    }
    pub(in crate::ud::item) fn set_effect_modes(
        &mut self,
        effect_modes: impl Iterator<Item = (AEffectId, EffectMode)>,
        reuse_eupdates: &mut UEffectUpdates,
        src: &Src,
    ) {
        self.base.set_effect_modes(effect_modes, reuse_eupdates, src)
    }
    pub(crate) fn is_loaded(&self) -> bool {
        self.base.is_loaded()
    }
    pub(in crate::ud::item) fn src_changed(&mut self, _reuse_eupdates: &mut UEffectUpdates, _src: &Src) {
        // Just panic to expose attempts to reload it, since autocharges should never be reloaded.
        // Instead, they are removed and re-added when parent item changes.
        unreachable!("autocharges should be removed/added outside of autocharge item handler");
    }
    // Item-specific methods
    pub(crate) fn get_activated(&self) -> bool {
        self.force_disabled
    }
    pub(crate) fn set_activated(&mut self, activated: bool, reuse_eupdates: &mut UEffectUpdates, src: &Src) {
        // No changes to state - nothing to do but clear reusable data
        if self.activated == activated {
            reuse_eupdates.clear();
            return;
        }
        self.activated = activated;
        self.base
            .set_state(get_state(self.activated, self.force_disabled), reuse_eupdates, src);
    }
    pub(crate) fn get_force_disabled(&self) -> bool {
        self.force_disabled
    }
    pub(crate) fn set_force_disabled(&mut self, force_disabled: bool, reuse_eupdates: &mut UEffectUpdates, src: &Src) {
        // No changes to state - nothing to do but clear reusable data
        if self.force_disabled == force_disabled {
            reuse_eupdates.clear();
            return;
        }
        self.force_disabled = force_disabled;
        self.base
            .set_state(get_state(self.activated, self.force_disabled), reuse_eupdates, src);
    }
    pub(crate) fn get_fit_key(&self) -> UFitKey {
        self.fit_key
    }
    pub(crate) fn get_cont_item_key(&self) -> UItemKey {
        self.cont_item_key
    }
    pub(crate) fn get_cont_effect_key(&self) -> REffectKey {
        self.cont_effect_key
    }
    pub(crate) fn get_projs(&self) -> &Projs {
        &self.projs
    }
    pub(crate) fn get_projs_mut(&mut self) -> &mut Projs {
        &mut self.projs
    }
}
impl Named for UAutocharge {
    fn get_name() -> &'static str {
        "Autocharge"
    }
}
impl std::fmt::Display for UAutocharge {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}(item_id={}, type_id={})",
            Self::get_name(),
            self.get_item_id(),
            self.get_type_id(),
        )
    }
}

fn get_state(active: bool, force_disable: bool) -> AState {
    match force_disable {
        true => AState::Ghost,
        false => match active {
            true => AState::Active,
            false => AState::Offline,
        },
    }
}
