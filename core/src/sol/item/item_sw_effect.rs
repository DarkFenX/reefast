use crate::{
    defs::{EItemId, SolItemId},
    sol::item::{bool_to_state, state_to_bool, SolItemBase, SolItemState},
    src::Src,
    util::Named,
};

pub(in crate::sol) struct SolSwEffect {
    pub(in crate::sol) base: SolItemBase,
    pub(in crate::sol) state: SolItemState,
}
impl SolSwEffect {
    pub(in crate::sol) fn new(src: &Src, id: SolItemId, a_item_id: EItemId, state: bool) -> Self {
        Self {
            base: SolItemBase::new(src, id, a_item_id),
            state: bool_to_state(state),
        }
    }
    pub(in crate::sol) fn get_bool_state(&self) -> bool {
        state_to_bool(self.state)
    }
    pub(in crate::sol) fn set_bool_state(&mut self, state: bool) {
        self.state = bool_to_state(state);
    }
}
impl Named for SolSwEffect {
    fn get_name() -> &'static str {
        "SolSwEffect"
    }
}
impl std::fmt::Display for SolSwEffect {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}(id={}, a_item_id={})",
            Self::get_name(),
            self.base.id,
            self.base.a_item_id
        )
    }
}
