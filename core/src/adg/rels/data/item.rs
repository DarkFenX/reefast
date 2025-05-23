use crate::{
    adg::{
        GSupport,
        rels::{Fk, KeyPart, Pk},
    },
    ed,
};

impl Pk for ed::EItem {
    fn get_pk(&self) -> Vec<KeyPart> {
        vec![self.id]
    }
}

impl Fk for ed::EItem {
    fn get_group_fks(&self, _: &GSupport) -> Vec<KeyPart> {
        vec![self.group_id]
    }
}
