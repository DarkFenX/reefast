use std::{fmt, sync::Arc};

use crate::{
    ct,
    defines::{ReeId, ReeInt},
    src::Src,
    util::Named,
};

pub struct CharacterInfo {
    pub item_id: ReeId,
    pub fit_id: ReeId,
    pub type_id: ReeInt,
}
impl CharacterInfo {
    fn new(item_id: ReeId, fit_id: ReeId, type_id: ReeInt) -> Self {
        Self {
            item_id,
            fit_id,
            type_id,
        }
    }
}
impl From<&Character> for CharacterInfo {
    fn from(c: &Character) -> Self {
        CharacterInfo::new(c.item_id, c.fit_id, c.type_id)
    }
}

pub(in crate::ss) struct Character {
    pub(in crate::ss) item_id: ReeId,
    pub(in crate::ss) fit_id: ReeId,
    pub(in crate::ss) type_id: ReeInt,
    pub(in crate::ss) citem: Option<Arc<ct::Item>>,
}
impl Character {
    pub(in crate::ss) fn new(src: &Arc<Src>, item_id: ReeId, fit_id: ReeId, type_id: ReeInt) -> Self {
        Self {
            item_id,
            fit_id,
            type_id,
            citem: src.cache_handler.get_item(&type_id),
        }
    }
}
impl Named for Character {
    fn get_name() -> &'static str {
        "ssi:Character"
    }
}
impl fmt::Display for Character {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}(id={}, type_id={})", Self::get_name(), self.item_id, self.type_id)
    }
}
