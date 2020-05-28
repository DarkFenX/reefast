use crate::{defines::ReeInt, dh};

use super::{Fk, Pk, Support};

impl Pk for dh::Item {
    fn get_pk(&self) -> Vec<ReeInt> {
        vec![self.id]
    }
}

impl Fk for dh::Item {
    fn get_item_fks(&self, _: &Support) -> Vec<ReeInt> {
        Vec::new()
    }
    fn get_item_group_fks(&self, _: &Support) -> Vec<ReeInt> {
        vec![self.group_id]
    }
    fn get_attr_fks(&self, _: &Support) -> Vec<ReeInt> {
        Vec::new()
    }
    fn get_effect_fks(&self, _: &Support) -> Vec<ReeInt> {
        Vec::new()
    }
    fn get_fighter_abil_fks(&self, _: &Support) -> Vec<ReeInt> {
        Vec::new()
    }
    fn get_buff_fks(&self, _: &Support) -> Vec<ReeInt> {
        Vec::new()
    }
}
