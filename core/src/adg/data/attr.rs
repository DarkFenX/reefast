use crate::{
    consts::{attrs, units},
    defs::ReeInt,
    edt, util,
};

use super::{aux, Fk, Pk, Support};

impl Pk for edt::EAttr {
    fn get_pk(&self) -> Vec<ReeInt> {
        vec![self.id]
    }
}

impl Fk for edt::EAttr {
    fn get_item_fks(&self, _: &Support) -> Vec<ReeInt> {
        let mut vec = Vec::new();
        if let Some(v) = self.get_fk_from_defval(units::ITEM_ID) {
            vec.push(v);
        }
        vec
    }
    fn get_group_fks(&self, _: &Support) -> Vec<ReeInt> {
        let mut vec = Vec::new();
        if let Some(v) = self.get_fk_from_defval(units::GROUP_ID) {
            vec.push(v);
        }
        vec
    }
    fn get_attr_fks(&self, _: &Support) -> Vec<ReeInt> {
        let mut vec = Vec::new();
        util::vec_push_opt(&mut vec, self.max_attr_id);
        if let Some(v) = self.get_fk_from_defval(units::ATTR_ID) {
            vec.push(v);
        }
        vec
    }
    fn get_buff_fks(&self, _: &Support) -> Vec<ReeInt> {
        let mut vec = Vec::new();
        if let (true, Some(dv_fk)) = (
            attrs::BUFF_ID_ATTRS.contains(&self.id),
            aux::attrval_to_fk(self.default_value),
        ) {
            vec.push(dv_fk);
        }
        vec
    }
}
impl edt::EAttr {
    /// Receive unit ID, and if the attribute has such unit ID - push its default value to the
    /// vector.
    fn get_fk_from_defval(&self, unit: ReeInt) -> Option<ReeInt> {
        match (self.unit_id, aux::attrval_to_fk(self.default_value)) {
            (Some(u), Some(dv_fk)) if u == unit => Some(dv_fk),
            _ => None,
        }
    }
}
