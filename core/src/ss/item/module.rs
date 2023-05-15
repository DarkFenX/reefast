use std::{fmt, sync::Arc};

use crate::{
    consts::State,
    ct,
    defs::{ReeId, ReeIdx, ReeInt},
    src::Src,
    util::Named,
};

pub(in crate::ss) struct Module {
    pub(in crate::ss) item_id: ReeId,
    pub(in crate::ss) fit_id: ReeId,
    pub(in crate::ss) type_id: ReeInt,
    pub(in crate::ss) state: State,
    pub(in crate::ss) pos: ReeIdx,
    pub(in crate::ss) charge: Option<ReeId>,
    pub(in crate::ss) citem: Option<Arc<ct::Item>>,
}
impl Module {
    pub(in crate::ss) fn new(
        src: &Arc<Src>,
        item_id: ReeId,
        fit_id: ReeId,
        type_id: ReeInt,
        state: State,
        pos: ReeIdx,
        charge: Option<ReeId>,
    ) -> Self {
        Self {
            item_id,
            fit_id,
            type_id,
            state,
            pos,
            charge,
            citem: src.cache_handler.get_item(&type_id),
        }
    }
}
impl Named for Module {
    fn get_name() -> &'static str {
        "ssi:Module"
    }
}
impl fmt::Display for Module {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}(id={}, type_id={})", Self::get_name(), self.item_id, self.type_id)
    }
}
