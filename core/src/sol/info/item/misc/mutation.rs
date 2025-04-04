use crate::sol::{AttrId, AttrVal, ItemTypeId, MutaRoll};

pub struct ItemMutationInfo {
    pub base_type_id: ItemTypeId,
    pub mutator_id: ItemTypeId,
    pub attrs: Vec<AttrMutationInfo>,
}

pub struct AttrMutationInfo {
    pub attr_id: AttrId,
    pub roll: Option<MutaRoll>,
    pub value: AttrVal,
}
