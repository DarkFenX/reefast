#[derive(serde::Serialize)]
pub(crate) struct HRigInfoPartial {
    #[serde(with = "crate::util::serde_string")]
    pub(crate) id: rc::SsItemId,
    #[serde(with = "crate::util::serde_string")]
    pub(crate) fit_id: rc::SsFitId,
    pub(crate) type_id: rc::ItemId,
    pub(crate) enabled: bool,
}
impl From<&rc::SsRigInfo> for HRigInfoPartial {
    fn from(core_rig_info: &rc::SsRigInfo) -> Self {
        Self {
            id: core_rig_info.id,
            fit_id: core_rig_info.fit_id,
            type_id: core_rig_info.a_item_id,
            enabled: core_rig_info.enabled,
        }
    }
}
