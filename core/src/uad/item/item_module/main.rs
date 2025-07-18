use crate::{
    ad,
    def::{Count, FitKey, Idx, ItemId, ItemKey, OF},
    err::basic::ItemNotMutatedError,
    misc::{AttrMutationRequest, EffectMode, ItemMutationRequest, ModRack, ModuleState, Spool},
    src::Src,
    uad::{
        Uad,
        err::ItemMutatedError,
        item::{ItemMutationData, Projs, UadEffectUpdates, UadItemBaseMutable},
    },
    util::{Named, RMap, RSet, trunc_unerr},
};

#[derive(Clone)]
pub(crate) struct UadModule {
    base: UadItemBaseMutable,
    fit_key: FitKey,
    rack: ModRack,
    pos: Idx,
    charge_key: Option<ItemKey>,
    spool: Option<Spool>,
    projs: Projs,
}
impl UadModule {
    pub(crate) fn new(
        item_id: ItemId,
        a_item_id: ad::AItemId,
        fit_key: FitKey,
        state: ModuleState,
        rack: ModRack,
        pos: Idx,
        mutation: Option<ItemMutationRequest>,
        charge_key: Option<ItemKey>,
        src: &Src,
        reuse_eupdates: &mut UadEffectUpdates,
    ) -> Self {
        Self {
            base: UadItemBaseMutable::new(item_id, a_item_id, state.into(), mutation, src, reuse_eupdates),
            fit_key,
            rack,
            pos,
            charge_key,
            spool: None,
            projs: Projs::new(),
        }
    }
    // Item base methods
    pub(crate) fn get_item_id(&self) -> ItemId {
        self.base.get_item_id()
    }
    pub(crate) fn get_a_item_id(&self) -> ad::AItemId {
        self.base.get_a_item_id()
    }
    pub(crate) fn set_a_item_id(&mut self, a_item_id: ad::AItemId, reuse_eupdates: &mut UadEffectUpdates, src: &Src) {
        self.base.set_a_item_id(a_item_id, reuse_eupdates, src);
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
    pub(crate) fn get_max_a_state(&self) -> Option<ad::AState> {
        self.base.get_max_a_state()
    }
    pub(crate) fn get_val_fitted_a_group_id(&self) -> Option<ad::AItemGrpId> {
        self.base.get_val_fitted_a_group_id()
    }
    pub(crate) fn get_val_online_a_group_id(&self) -> Option<ad::AItemGrpId> {
        self.base.get_val_online_a_group_id()
    }
    pub(crate) fn get_val_active_a_group_id(&self) -> Option<ad::AItemGrpId> {
        self.base.get_val_active_a_group_id()
    }
    pub(crate) fn get_a_state(&self) -> ad::AState {
        self.base.get_a_state()
    }
    pub(crate) fn get_reffs(&self) -> Option<&RSet<ad::AEffectId>> {
        self.base.get_reffs()
    }
    pub(in crate::uad::item) fn start_all_reffs(&self, reuse_eupdates: &mut UadEffectUpdates, src: &Src) {
        self.base.start_all_reffs(reuse_eupdates, src);
    }
    pub(in crate::uad::item) fn stop_all_reffs(&self, reuse_eupdates: &mut UadEffectUpdates, src: &Src) {
        self.base.stop_all_reffs(reuse_eupdates, src)
    }
    pub(in crate::uad::item) fn get_effect_mode(&self, effect_id: &ad::AEffectId) -> EffectMode {
        self.base.get_effect_mode(effect_id)
    }
    pub(in crate::uad::item) fn set_effect_mode(
        &mut self,
        a_effect_id: ad::AEffectId,
        effect_mode: EffectMode,
        reuse_eupdates: &mut UadEffectUpdates,
        src: &Src,
    ) {
        self.base.set_effect_mode(a_effect_id, effect_mode, reuse_eupdates, src)
    }
    pub(in crate::uad::item) fn set_effect_modes(
        &mut self,
        modes: impl Iterator<Item = (ad::AEffectId, EffectMode)>,
        reuse_eupdates: &mut UadEffectUpdates,
        src: &Src,
    ) {
        self.base.set_effect_modes(modes, reuse_eupdates, src)
    }
    pub(crate) fn is_loaded(&self) -> bool {
        self.base.is_loaded()
    }
    pub(in crate::uad::item) fn update_a_data(&mut self, reuse_eupdates: &mut UadEffectUpdates, src: &Src) {
        self.base.update_a_data(reuse_eupdates, src);
    }
    // Mutation-specific methods
    pub(crate) fn get_mutation_data(&self) -> Option<&ItemMutationData> {
        self.base.get_mutation_data()
    }
    pub(crate) fn mutate(
        &mut self,
        mutation: ItemMutationRequest,
        reuse_eupdates: &mut UadEffectUpdates,
        src: &Src,
    ) -> Result<(), ItemNotMutatedError> {
        self.base.mutate(mutation, reuse_eupdates, src)
    }
    pub(crate) fn change_mutation_attrs(
        &mut self,
        src: &Src,
        attr_mutations: Vec<AttrMutationRequest>,
    ) -> Result<Vec<ad::AAttrId>, ItemMutatedError> {
        self.base.change_mutation_attrs(src, attr_mutations)
    }
    pub(crate) fn set_a_mutator_id(
        &mut self,
        a_mutator_id: ad::AItemId,
        reuse_eupdates: &mut UadEffectUpdates,
        src: &Src,
    ) -> Result<(), ItemMutatedError> {
        self.base.set_a_mutator_id(a_mutator_id, reuse_eupdates, src)
    }
    pub(crate) fn unmutate(
        &mut self,
        reuse_eupdates: &mut UadEffectUpdates,
        src: &Src,
    ) -> Result<(), ItemMutatedError> {
        self.base.unmutate(reuse_eupdates, src)
    }
    // Item-specific methods
    pub(crate) fn get_module_state(&self) -> ModuleState {
        self.base.get_a_state().into()
    }
    pub(crate) fn set_module_state(&mut self, state: ModuleState, reuse_eupdates: &mut UadEffectUpdates, src: &Src) {
        self.base.set_a_state(state.into(), reuse_eupdates, src)
    }
    pub(crate) fn get_fit_key(&self) -> FitKey {
        self.fit_key
    }
    pub(crate) fn get_rack(&self) -> ModRack {
        self.rack
    }
    pub(crate) fn get_pos(&self) -> Idx {
        self.pos
    }
    pub(crate) fn set_pos(&mut self, pos: Idx) {
        self.pos = pos
    }
    pub(crate) fn get_charge_key(&self) -> Option<ItemKey> {
        self.charge_key
    }
    pub(crate) fn set_charge_key(&mut self, charge_key: Option<ItemKey>) {
        self.charge_key = charge_key
    }
    pub(crate) fn get_charge_count(&self, uad: &Uad) -> Option<Count> {
        // No charge - no info
        let charge_key = self.get_charge_key()?;
        let charge_item = uad.items.get(charge_key);
        let module_capacity = match self.get_a_xt() {
            Some(a_xt) => a_xt.capacity,
            // Module not loaded - no info
            _ => {
                return None;
            }
        };
        let charge_volume = match charge_item.get_a_xt() {
            Some(a_xt) if a_xt.volume != OF(0.0) => a_xt.volume,
            // Charge not loaded or has 0 volume - no info
            _ => {
                return None;
            }
        };
        let charge_count = trunc_unerr(module_capacity / charge_volume) as Count;
        Some(charge_count)
    }
    pub(crate) fn get_spool(&self) -> Option<Spool> {
        self.spool
    }
    pub(crate) fn set_spool(&mut self, spool: Option<Spool>) {
        self.spool = spool
    }
    pub(crate) fn get_projs(&self) -> &Projs {
        &self.projs
    }
    pub(crate) fn get_projs_mut(&mut self) -> &mut Projs {
        &mut self.projs
    }
}
impl Named for UadModule {
    fn get_name() -> &'static str {
        "Module"
    }
}
impl std::fmt::Display for UadModule {
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
