use crate::{
    ad,
    defs::{AttrVal, EAttrId, EEffectId, EItemGrpId, EItemId, SkillLevel, SolItemId},
    err::basic::ItemLoadedError,
    sol::item::{SolEffectModes, SolItemState},
    src::Src,
    util::StMap,
};

// Item base stores all the data every item should have
#[derive(Clone)]
pub(in crate::sol::item) struct SolItemBase {
    // User-defined data
    id: SolItemId,
    type_id: EItemId,
    state: SolItemState,
    effect_modes: SolEffectModes,
    // Source-dependent data
    cache: Option<SolItemBaseCache>,
}
impl SolItemBase {
    pub(in crate::sol::item) fn new(src: &Src, id: SolItemId, type_id: EItemId, state: SolItemState) -> Self {
        Self {
            id,
            type_id,
            state,
            effect_modes: SolEffectModes::new(),
            cache: src.get_a_item(&type_id).map(|v| SolItemBaseCache::new(v.clone())),
        }
    }
    pub(in crate::sol::item) fn get_id(&self) -> SolItemId {
        self.id
    }
    pub(in crate::sol::item) fn get_type_id(&self) -> EItemId {
        self.type_id
    }
    pub(in crate::sol::item) fn get_group_id(&self) -> Result<EItemGrpId, ItemLoadedError> {
        self.get_a_item().map(|v| v.grp_id)
    }
    pub(in crate::sol::item) fn get_category_id(&self) -> Result<EItemGrpId, ItemLoadedError> {
        self.get_a_item().map(|v| v.cat_id)
    }
    pub(in crate::sol::item) fn get_attrs(&self) -> Result<&StMap<EAttrId, AttrVal>, ItemLoadedError> {
        self.get_a_item().map(|v| &v.attr_vals)
    }
    pub(in crate::sol::item) fn get_effect_datas(
        &self,
    ) -> Result<&StMap<EEffectId, ad::AItemEffectData>, ItemLoadedError> {
        self.get_a_item().map(|v| &v.effect_datas)
    }
    pub(in crate::sol::item) fn get_defeff_id(&self) -> Result<Option<EEffectId>, ItemLoadedError> {
        self.get_a_item().map(|v| v.defeff_id)
    }
    pub(in crate::sol::item) fn get_skill_reqs(&self) -> Result<&StMap<EItemId, SkillLevel>, ItemLoadedError> {
        self.get_a_item().map(|v| &v.srqs)
    }
    pub(in crate::sol::item) fn get_state(&self) -> SolItemState {
        self.state
    }
    pub(in crate::sol::item) fn set_state(&mut self, state: SolItemState) {
        self.state = state
    }
    pub(in crate::sol::item) fn get_effect_modes(&self) -> &SolEffectModes {
        &self.effect_modes
    }
    pub(in crate::sol::item) fn get_effect_modes_mut(&mut self) -> &mut SolEffectModes {
        &mut self.effect_modes
    }
    pub(in crate::sol::item) fn is_loaded(&self) -> bool {
        self.cache.is_some()
    }
    pub(in crate::sol::item) fn update_a_data(&mut self, src: &Src) {
        self.cache = src.get_a_item(&self.type_id).map(|v| SolItemBaseCache::new(v.clone()));
    }
    // Non-public methods
    pub(in crate::sol::item::base) fn set_type_id(&mut self, type_id: EItemId) {
        self.type_id = type_id;
    }
    pub(in crate::sol::item::base) fn set_a_item(&mut self, a_item: ad::ArcItem) {
        match &mut self.cache {
            Some(cache) => cache.a_item = a_item,
            None => self.cache = Some(SolItemBaseCache::new(a_item)),
        }
    }
    pub(in crate::sol::item::base) fn remove_a_item(&mut self) {
        self.cache = None;
    }
    fn get_a_item(&self) -> Result<&ad::ArcItem, ItemLoadedError> {
        match &self.cache {
            Some(cache) => Ok(&cache.a_item),
            None => Err(ItemLoadedError::new(self.id)),
        }
    }
}

#[derive(Clone)]
struct SolItemBaseCache {
    a_item: ad::ArcItem,
}
impl SolItemBaseCache {
    fn new(a_item: ad::ArcItem) -> Self {
        Self { a_item }
    }
}
