use crate::{
    ad,
    defs::{EItemId, SlotNumber, SsFitId, SsItemId},
    ec,
    shr::State,
    src::Src,
    util::Named,
};

use super::support::{bool_to_state, state_to_bool, EffectModes};

pub(in crate::ss) struct SsImplant {
    pub(in crate::ss) id: SsItemId,
    pub(in crate::ss) fit_id: SsFitId,
    pub(in crate::ss) a_item_id: EItemId,
    pub(in crate::ss) state: State,
    pub(in crate::ss) effect_modes: EffectModes,
    pub(in crate::ss) a_item: Option<ad::ArcItem>,
}
impl SsImplant {
    pub(in crate::ss) fn new(src: &Src, id: SsItemId, fit_id: SsFitId, a_item_id: EItemId, state: bool) -> Self {
        Self {
            id,
            fit_id,
            a_item_id,
            state: bool_to_state(state),
            effect_modes: EffectModes::new(),
            a_item: src.get_a_item(&a_item_id),
        }
    }
    pub(in crate::ss) fn get_bool_state(&self) -> bool {
        state_to_bool(self.state)
    }
    pub(in crate::ss) fn set_bool_state(&mut self, state: bool) {
        self.state = bool_to_state(state);
    }
    pub(in crate::ss) fn get_slot(&self) -> Option<SlotNumber> {
        match &self.a_item {
            None => None,
            Some(a_item) => match a_item.attr_vals.get(&ec::attrs::IMPLANTNESS) {
                None => None,
                Some(value) => Some(value.round() as SlotNumber),
            },
        }
    }
}
impl Named for SsImplant {
    fn get_name() -> &'static str {
        "SsImplant"
    }
}
impl std::fmt::Display for SsImplant {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}(id={}, a_item_id={})", Self::get_name(), self.id, self.a_item_id)
    }
}
