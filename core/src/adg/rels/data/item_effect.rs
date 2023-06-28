use crate::{
    adg::{
        rels::{Fk, KeyPart, Pk},
        GSupport,
    },
    ed,
};

impl Pk for ed::EItemEffect {
    fn get_pk(&self) -> Vec<KeyPart> {
        vec![self.item_id, self.effect_id]
    }
}

impl Fk for ed::EItemEffect {
    fn get_item_fks(&self, _: &GSupport) -> Vec<KeyPart> {
        vec![self.item_id]
    }
    fn get_effect_fks(&self, _: &GSupport) -> Vec<KeyPart> {
        vec![self.effect_id]
    }
}
