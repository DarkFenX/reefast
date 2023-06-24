#[derive(serde::Serialize)]
pub(crate) struct HStanceInfoPartial {
    #[serde(with = "crate::util::serde_string")]
    pub(crate) id: rc::ReeId,
    #[serde(with = "crate::util::serde_string")]
    pub(crate) fit_id: rc::ReeId,
    pub(crate) type_id: rc::ReeInt,
    pub(crate) enabled: bool,
}
impl From<&rc::SsStanceInfo> for HStanceInfoPartial {
    fn from(core_stance_info: &rc::SsStanceInfo) -> Self {
        Self {
            id: core_stance_info.id,
            fit_id: core_stance_info.fit_id,
            type_id: core_stance_info.a_item_id,
            enabled: core_stance_info.enabled,
        }
    }
}
