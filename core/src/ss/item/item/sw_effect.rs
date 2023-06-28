use crate::{
    ad,
    consts::{EffectMode, State},
    defs::{EffectId, ItemId, SsItemId},
    src::Src,
    util::{Named, OptMap},
};

use super::{bool_to_state, state_to_bool};

pub(in crate::ss) struct SsSwEffect {
    pub(in crate::ss) id: SsItemId,
    pub(in crate::ss) a_item_id: ItemId,
    pub(in crate::ss) state: State,
    pub(in crate::ss) effect_modes: OptMap<EffectId, EffectMode>,
    pub(in crate::ss) a_item: Option<ad::ArcItem>,
}
impl SsSwEffect {
    pub(in crate::ss) fn new(src: &Src, id: SsItemId, a_item_id: ItemId, state: bool) -> Self {
        Self {
            id,
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
impl Named for SsSwEffect {
    fn get_name() -> &'static str {
        "SsSwEffect"
    }
}
impl std::fmt::Display for SsSwEffect {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}(id={}, a_item_id={})", Self::get_name(), self.id, self.a_item_id)
    }
}
