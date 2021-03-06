use crate::{defines::ReeInt, dh};

use super::{Fk, Pk, Support};

impl Pk for dh::ItemFighterAbil {
    fn get_pk(&self) -> Vec<ReeInt> {
        vec![self.item_id, self.abil_id]
    }
}

impl Fk for dh::ItemFighterAbil {
    fn get_item_fks(&self, _: &Support) -> Vec<ReeInt> {
        vec![self.item_id]
    }
    fn get_abil_fks(&self, _: &Support) -> Vec<ReeInt> {
        vec![self.abil_id]
    }
}
