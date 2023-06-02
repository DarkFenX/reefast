use std::collections::HashMap;

use crate::fsd::FsdMerge;

#[derive(Debug, serde::Deserialize)]
pub(crate) struct MutaAttrMods {
    #[serde(rename = "attributeIDs")]
    pub(crate) attrs: HashMap<rc::ReeInt, MutaAttrModRange>,
}
impl FsdMerge<rc::edt::EMutaAttrMod> for MutaAttrMods {
    fn fsd_merge(self, id: rc::ReeInt) -> Vec<rc::edt::EMutaAttrMod> {
        self.attrs
            .into_iter()
            .map(|(attr_id, range)| rc::edt::EMutaAttrMod::new(id, attr_id, range.min, range.max))
            .collect()
    }
}

#[derive(Debug, serde::Deserialize)]
pub(crate) struct MutaAttrModRange {
    pub(crate) min: rc::ReeFloat,
    pub(crate) max: rc::ReeFloat,
}
