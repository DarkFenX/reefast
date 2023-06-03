use std::{fmt, sync::Arc};

use crate::{
    ad,
    consts::{attrs, State},
    defs::{ReeId, ReeInt},
    src::Src,
    util::Named,
};

use super::{bool_to_state, state_to_bool};

pub(crate) struct Subsystem {
    pub(crate) id: ReeId,
    pub(crate) fit_id: ReeId,
    pub(crate) type_id: ReeInt,
    pub(crate) state: State,
    pub(crate) cached_item: Option<Arc<ad::AItem>>,
}
impl Subsystem {
    pub(crate) fn new(src: &Arc<Src>, id: ReeId, fit_id: ReeId, type_id: ReeInt, state: bool) -> Self {
        Self {
            id,
            fit_id,
            type_id,
            state: bool_to_state(state),
            cached_item: src.cache_handler.get_item(&type_id),
        }
    }
    pub(crate) fn get_bool_state(&self) -> bool {
        state_to_bool(self.state)
    }
    pub(crate) fn set_bool_state(&mut self, state: bool) {
        self.state = bool_to_state(state);
    }
    pub(crate) fn get_slot(&self) -> Option<ReeInt> {
        match &self.cached_item {
            None => None,
            Some(i) => match i.attr_vals.get(&attrs::SUBSYSTEM_SLOT) {
                None => None,
                Some(v) => Some(v.round() as ReeInt),
            },
        }
    }
}
impl Named for Subsystem {
    fn get_name() -> &'static str {
        "ssi:Subsystem"
    }
}
impl fmt::Display for Subsystem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}(id={}, type_id={})", Self::get_name(), self.id, self.type_id)
    }
}
