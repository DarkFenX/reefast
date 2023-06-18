#[derive(Debug, serde_tuple::Serialize_tuple, serde_tuple::Deserialize_tuple)]
pub(crate) struct HAttrVal {
    pub(crate) base: rc::ReeFloat,
    pub(crate) dogma: rc::ReeFloat,
    pub(crate) extra: rc::ReeFloat,
}
impl HAttrVal {
    fn new(base: rc::ReeFloat, dogma: rc::ReeFloat, extra: rc::ReeFloat) -> Self {
        Self { base, dogma, extra }
    }
}
impl From<&rc::SsAttrVal> for HAttrVal {
    fn from(core_attr_val: &rc::SsAttrVal) -> Self {
        Self::new(core_attr_val.base, core_attr_val.dogma, core_attr_val.extra)
    }
}
