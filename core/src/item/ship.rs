use std::sync::Arc;

use crate::{ct, ReeId, ReeInt, Src};

pub(crate) struct Ship {
    pub(crate) id: ReeId,
    pub(crate) fit_id: ReeId,
    pub(crate) type_id: ReeInt,
    pub(crate) item: Option<Arc<ct::Item>>,
}
impl Ship {
    pub fn new(src: Arc<Src>, id: ReeId, fit_id: ReeId, type_id: ReeInt) -> Ship {
        Ship {
            id,
            fit_id,
            type_id,
            item: src.cache_handler.get_item(type_id),
        }
    }
}
