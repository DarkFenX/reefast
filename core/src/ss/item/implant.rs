use std::{fmt, sync::Arc};

use crate::{
    consts::{attrs, State},
    ct,
    defs::{ReeId, ReeInt},
    src::Src,
    util::Named,
};

pub(in crate::ss) struct Implant {
    pub(in crate::ss) item_id: ReeId,
    pub(in crate::ss) fit_id: ReeId,
    pub(in crate::ss) type_id: ReeInt,
    pub(in crate::ss) state: State,
    pub(in crate::ss) cached_item: Option<Arc<ct::Item>>,
}
impl Implant {
    pub(in crate::ss) fn new(src: &Arc<Src>, item_id: ReeId, fit_id: ReeId, type_id: ReeInt) -> Self {
        Self {
            item_id,
            fit_id,
            type_id,
            state: State::Offline,
            cached_item: src.cache_handler.get_item(&type_id),
        }
    }
    pub(in crate::ss) fn get_bool_state(&self) -> bool {
        match self.state {
            State::Ghost => false,
            _ => true,
        }
    }
    pub(in crate::ss) fn set_bool_state(&mut self, state: bool) {
        self.state = match state {
            true => State::Offline,
            false => State::Ghost,
        }
    }
    pub(in crate::ss) fn get_slot(&self) -> Option<ReeInt> {
        match &self.cached_item {
            None => None,
            Some(i) => match i.attr_vals.get(&attrs::IMPLANTNESS) {
                None => None,
                Some(v) => Some(v.round() as ReeInt),
            },
        }
    }
}
impl Named for Implant {
    fn get_name() -> &'static str {
        "ssi:Implant"
    }
}
impl fmt::Display for Implant {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}(id={}, type_id={})", Self::get_name(), self.item_id, self.type_id)
    }
}
