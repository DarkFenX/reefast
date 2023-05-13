use std::{fmt, sync::Arc};

use crate::{
    consts::{attrs, State},
    ct,
    defines::{ReeId, ReeInt},
    src::Src,
    util::Named,
};

pub struct BoosterInfo {
    pub item_id: ReeId,
    pub fit_id: ReeId,
    pub type_id: ReeInt,
    pub state: bool,
}
impl BoosterInfo {
    fn new(item_id: ReeId, fit_id: ReeId, type_id: ReeInt, state: bool) -> Self {
        Self {
            item_id,
            fit_id,
            type_id,
            state,
        }
    }
}
impl From<&Booster> for BoosterInfo {
    fn from(b: &Booster) -> Self {
        BoosterInfo::new(b.item_id, b.fit_id, b.type_id, b.get_bool_state())
    }
}

pub(in crate::ss) struct Booster {
    pub(in crate::ss) item_id: ReeId,
    pub(in crate::ss) fit_id: ReeId,
    pub(in crate::ss) type_id: ReeInt,
    pub(in crate::ss) state: State,
    pub(in crate::ss) citem: Option<Arc<ct::Item>>,
}
impl Booster {
    pub(in crate::ss) fn new(src: &Arc<Src>, item_id: ReeId, fit_id: ReeId, type_id: ReeInt) -> Self {
        Self {
            item_id,
            fit_id,
            type_id,
            state: State::Offline,
            citem: src.cache_handler.get_item(&type_id),
        }
    }
    pub(in crate::ss) fn get_slot(&self) -> Option<ReeInt> {
        match &self.citem {
            None => None,
            Some(i) => match i.attr_vals.get(&attrs::BOOSTERNESS) {
                None => None,
                Some(v) => Some(v.round() as ReeInt),
            },
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
}
impl Named for Booster {
    fn get_name() -> &'static str {
        "ssi:Booster"
    }
}
impl fmt::Display for Booster {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}(id={}, type_id={})", Self::get_name(), self.item_id, self.type_id)
    }
}
