use crate::{
    defs::{AttrVal, EAttrId, EMutaId},
    util::Named,
};

/// EVE mutaplasmid attribute modification data.
#[derive(Debug)]
pub struct EMutaAttrMod {
    /// Mutaplasmid item type ID.
    pub muta_id: EMutaId,
    /// Refers an attribute being modified by the mutaplasmid.
    pub attr_id: EAttrId,
    /// Lower boundary of the modification range.
    pub min_attr_mult: AttrVal,
    /// Upper boundary of the modification range.
    pub max_attr_mult: AttrVal,
}
impl EMutaAttrMod {
    /// Make a new EVE mutaplasmid attribute conversion.
    pub fn new(muta_id: EMutaId, attr_id: EAttrId, min_attr_mult: AttrVal, max_attr_mult: AttrVal) -> Self {
        Self {
            muta_id,
            attr_id,
            min_attr_mult,
            max_attr_mult,
        }
    }
}
impl Named for EMutaAttrMod {
    fn get_name() -> &'static str {
        "EMutaAttrMod"
    }
}
