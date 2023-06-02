use crate::{defs::ReeInt, edt};

use super::{Fk, Pk, Support};

impl Pk for edt::EItemSkillReq {
    fn get_pk(&self) -> Vec<ReeInt> {
        vec![self.item_id, self.skill_id]
    }
}

impl Fk for edt::EItemSkillReq {
    fn get_item_fks(&self, _: &Support) -> Vec<ReeInt> {
        vec![self.item_id, self.skill_id]
    }
}
