use std::fmt;

use crate::{
    ad,
    consts::{EffectMode, State},
    defs::{EffectId, ReeId, ReeInt},
    src::Src,
    util::{Named, OptMap},
};

use super::{bool_to_state, state_to_bool};

pub(crate) struct SsStance {
    pub(crate) id: ReeId,
    pub(crate) fit_id: ReeId,
    pub(crate) a_item_id: ReeInt,
    pub(crate) state: State,
    pub(crate) effect_modes: OptMap<EffectId, EffectMode>,
    pub(crate) a_item: Option<ad::ArcItem>,
}
impl SsStance {
    pub(crate) fn new(src: &Src, id: ReeId, fit_id: ReeId, a_item_id: ReeInt, state: bool) -> Self {
        Self {
            id,
            fit_id,
            a_item_id,
            state: bool_to_state(state),
            effect_modes: OptMap::new(),
            a_item: src.get_a_item(&a_item_id),
        }
    }
    pub(crate) fn get_bool_state(&self) -> bool {
        state_to_bool(self.state)
    }
    pub(crate) fn set_bool_state(&mut self, state: bool) {
        self.state = bool_to_state(state);
    }
}
impl Named for SsStance {
    fn get_name() -> &'static str {
        "SsStance"
    }
}
impl fmt::Display for SsStance {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}(id={}, a_item_id={})", Self::get_name(), self.id, self.a_item_id)
    }
}
