use crate::defines::ReeInt;
use crate::dh;

use super::{Fk, Pk};

impl Pk for dh::Attr {
    fn get_pk(&self) -> Vec<ReeInt> {
        vec![self.id]
    }
}

impl Fk for dh::Attr {
    fn get_item_fks(&self) -> Vec<ReeInt> {
        Vec::new()
    }
    fn get_item_group_fks(&self) -> Vec<ReeInt> {
        Vec::new()
    }
    fn get_attr_fks(&self) -> Vec<ReeInt> {
        Vec::new()
    }
    fn get_effect_fks(&self) -> Vec<ReeInt> {
        Vec::new()
    }
    fn get_fighter_abil_fks(&self) -> Vec<ReeInt> {
        Vec::new()
    }
    fn get_buff_fks(&self) -> Vec<ReeInt> {
        Vec::new()
    }
    fn is_mn_map() -> bool {
        false
    }
}
