use crate::{
    ad,
    consts::{EffectMode, State},
    defs::{EEffectId, EItemId, SsFitId, SsItemId},
    src::Src,
    util::{Named, OptMap},
};

use super::support::{bool_to_state, state_to_bool};

pub(in crate::ss) struct SsStance {
    pub(in crate::ss) id: SsItemId,
    pub(in crate::ss) fit_id: SsFitId,
    pub(in crate::ss) a_item_id: EItemId,
    pub(in crate::ss) state: State,
    pub(in crate::ss) effect_modes: OptMap<EEffectId, EffectMode>,
    pub(in crate::ss) a_item: Option<ad::ArcItem>,
}
impl SsStance {
    pub(in crate::ss) fn new(src: &Src, id: SsItemId, fit_id: SsFitId, a_item_id: EItemId, state: bool) -> Self {
        Self {
            id,
            fit_id,
            a_item_id,
            state: bool_to_state(state),
            effect_modes: OptMap::new(),
            a_item: src.get_a_item(&a_item_id),
        }
    }
    pub(in crate::ss) fn get_bool_state(&self) -> bool {
        state_to_bool(self.state)
    }
    pub(in crate::ss) fn set_bool_state(&mut self, state: bool) {
        self.state = bool_to_state(state);
    }
}
impl Named for SsStance {
    fn get_name() -> &'static str {
        "SsStance"
    }
}
impl std::fmt::Display for SsStance {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}(id={}, a_item_id={})", Self::get_name(), self.id, self.a_item_id)
    }
}
