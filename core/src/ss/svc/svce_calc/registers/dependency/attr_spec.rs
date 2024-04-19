use crate::defs::{EAttrId, SsItemId};

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub(in crate::ss::svc::svce_calc) struct SsAttrSpec {
    pub(in crate::ss::svc::svce_calc) item_id: SsItemId,
    pub(in crate::ss::svc::svce_calc) attr_id: EAttrId,
}
impl SsAttrSpec {
    pub(super) fn new(item_id: SsItemId, attr_id: EAttrId) -> Self {
        Self { item_id, attr_id }
    }
}
