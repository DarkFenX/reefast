use crate::{
    ad,
    defs::{Amount, EItemId, SolFitId, SolItemId},
    err::basic::ItemLoadedError,
    sol::item::{SolAutocharges, SolEffectModes, SolItemBase, SolItemState},
    src::Src,
    util::Named,
};

#[derive(Clone)]
pub(in crate::sol) struct SolFighter {
    base: SolItemBase,
    fit_id: SolFitId,
    amt_override: Option<Amount>,
    autocharges: SolAutocharges,
}
impl SolFighter {
    pub(in crate::sol) fn new(
        src: &Src,
        id: SolItemId,
        fit_id: SolFitId,
        type_id: EItemId,
        state: SolItemState,
    ) -> Self {
        Self {
            base: SolItemBase::new(src, id, type_id, state),
            fit_id,
            amt_override: None,
            autocharges: SolAutocharges::new(),
        }
    }
    // Item base methods
    pub(in crate::sol) fn get_id(&self) -> SolItemId {
        self.base.get_id()
    }
    pub(in crate::sol) fn get_type_id(&self) -> EItemId {
        self.base.get_type_id()
    }
    pub(in crate::sol) fn get_a_item(&self) -> Result<&ad::ArcItem, ItemLoadedError> {
        self.base.get_a_item()
    }
    pub(in crate::sol) fn get_state(&self) -> SolItemState {
        self.base.get_state()
    }
    pub(in crate::sol) fn set_state(&mut self, state: SolItemState) {
        self.base.set_state(state)
    }
    pub(in crate::sol) fn get_effect_modes(&self) -> &SolEffectModes {
        self.base.get_effect_modes()
    }
    pub(in crate::sol) fn get_effect_modes_mut(&mut self) -> &mut SolEffectModes {
        self.base.get_effect_modes_mut()
    }
    pub(in crate::sol) fn is_loaded(&self) -> bool {
        self.base.is_loaded()
    }
    pub(in crate::sol::item) fn reload_a_item(&mut self, src: &Src) {
        self.base.reload_a_item(src);
        self.autocharges.clear();
    }
    // Item-specific methods
    pub(in crate::sol) fn get_fit_id(&self) -> SolFitId {
        self.fit_id
    }
    pub(in crate::sol) fn get_amt_override(&self) -> Option<Amount> {
        self.amt_override
    }
    pub(in crate::sol) fn get_autocharges(&self) -> &SolAutocharges {
        &self.autocharges
    }
    pub(in crate::sol) fn get_autocharges_mut(&mut self) -> &mut SolAutocharges {
        &mut self.autocharges
    }
}
impl Named for SolFighter {
    fn get_name() -> &'static str {
        "SolFighter"
    }
}
impl std::fmt::Display for SolFighter {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}(id={}, type_id={})",
            Self::get_name(),
            self.get_id(),
            self.get_type_id(),
        )
    }
}
