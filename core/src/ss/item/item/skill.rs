use crate::{
    ad,
    consts::{EffectMode, State},
    defs::{EffectId, ReeId, ReeInt},
    src::Src,
    util::{Named, OptMap},
};

use super::{bool_to_state, state_to_bool};

pub(in crate::ss) struct SsSkill {
    pub(in crate::ss) id: ReeId,
    pub(in crate::ss) fit_id: ReeId,
    pub(in crate::ss) a_item_id: ReeInt,
    pub(in crate::ss) level: ReeInt,
    pub(in crate::ss) state: State,
    pub(in crate::ss) effect_modes: OptMap<EffectId, EffectMode>,
    pub(in crate::ss) a_item: Option<ad::ArcItem>,
}
impl SsSkill {
    pub(in crate::ss) fn new(
        src: &Src,
        id: ReeId,
        fit_id: ReeId,
        a_item_id: ReeInt,
        level: ReeInt,
        state: bool,
    ) -> Self {
        Self {
            id,
            fit_id,
            a_item_id,
            level,
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
impl Named for SsSkill {
    fn get_name() -> &'static str {
        "SsSkill"
    }
}
impl std::fmt::Display for SsSkill {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}(id={}, a_item_id={})", Self::get_name(), self.id, self.a_item_id)
    }
}
