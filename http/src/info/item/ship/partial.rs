#[derive(serde::Serialize)]
pub(crate) struct HShipInfoPartial {
    #[serde(with = "crate::util::serde_string")]
    pub(crate) id: rc::SsItemId,
    #[serde(with = "crate::util::serde_string")]
    pub(crate) fit_id: rc::SsFitId,
    pub(crate) type_id: rc::ItemId,
    pub(crate) enabled: bool,
}
impl From<&rc::SsShipInfo> for HShipInfoPartial {
    fn from(core_ship_info: &rc::SsShipInfo) -> Self {
        Self {
            id: core_ship_info.id,
            fit_id: core_ship_info.fit_id,
            type_id: core_ship_info.a_item_id,
            enabled: core_ship_info.enabled,
        }
    }
}
