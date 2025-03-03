use crate::{
    adg::{
        GSupport,
        rels::{Fk, KeyPart, Pk},
    },
    ed,
};

impl Pk for ed::EMutaAttrMod {
    fn get_pk(&self) -> Vec<KeyPart> {
        vec![self.muta_id, self.attr_id]
    }
}

impl Fk for ed::EMutaAttrMod {
    fn get_item_fks(&self, _: &GSupport) -> Vec<KeyPart> {
        vec![self.muta_id]
    }
    fn get_attr_fks(&self, _: &GSupport) -> Vec<KeyPart> {
        vec![self.attr_id]
    }
}
