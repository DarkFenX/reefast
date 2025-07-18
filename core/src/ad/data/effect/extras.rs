use crate::ad::AAttrId;

// Extra data attached to effect during runtime
pub(crate) struct AEffectXt {
    pub(crate) is_active: bool = false,
    pub(crate) proj_a_attr_ids: [Option<AAttrId>; 2] = [None, None],
}
