use crate::{
    ad,
    defs::{EItemId, SsItemId},
    src::Src,
    ss::item::{bool_to_state, state_to_bool, SsEffectModes, SsItemState, SsTgtItems},
    util::Named,
};

pub(in crate::ss) struct SsProjEffect {
    pub(in crate::ss) id: SsItemId,
    pub(in crate::ss) a_item_id: EItemId,
    pub(in crate::ss) state: SsItemState,
    pub(in crate::ss) tgts: SsTgtItems,
    pub(in crate::ss) effect_modes: SsEffectModes,
    pub(in crate::ss) a_item: Option<ad::ArcItem>,
}
impl SsProjEffect {
    pub(in crate::ss) fn new(src: &Src, id: SsItemId, a_item_id: EItemId, state: bool) -> Self {
        Self {
            id,
            a_item_id,
            state: bool_to_state(state),
            tgts: SsTgtItems::new(),
            effect_modes: SsEffectModes::new(),
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
impl Named for SsProjEffect {
    fn get_name() -> &'static str {
        "SsProjEffect"
    }
}
impl std::fmt::Display for SsProjEffect {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}(id={}, a_item_id={})", Self::get_name(), self.id, self.a_item_id)
    }
}
