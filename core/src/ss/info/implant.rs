use crate::{
    defs::{ReeId, ReeInt},
    ss::item::Implant,
};

pub struct ImplantInfo {
    pub item_id: ReeId,
    pub fit_id: ReeId,
    pub type_id: ReeInt,
    pub enabled: bool,
}
impl ImplantInfo {
    fn new(item_id: ReeId, fit_id: ReeId, type_id: ReeInt, enabled: bool) -> Self {
        Self {
            item_id,
            fit_id,
            type_id,
            enabled,
        }
    }
}
impl From<&Implant> for ImplantInfo {
    fn from(i: &Implant) -> Self {
        ImplantInfo::new(i.item_id, i.fit_id, i.type_id, i.get_bool_state())
    }
}
