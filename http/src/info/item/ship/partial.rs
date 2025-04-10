#[serde_with::serde_as]
#[derive(serde::Serialize)]
pub(crate) struct HShipInfoPartial {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub(crate) id: rc::ItemId,
    pub(crate) kind: &'static str,
    pub(crate) type_id: rc::ItemTypeId,
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub(crate) fit_id: rc::FitId,
    pub(crate) enabled: bool,
}
impl From<&rc::ShipInfo> for HShipInfoPartial {
    fn from(core_ship_info: &rc::ShipInfo) -> Self {
        Self {
            id: core_ship_info.id,
            kind: "ship",
            type_id: core_ship_info.type_id,
            fit_id: core_ship_info.fit_id,
            enabled: core_ship_info.enabled,
        }
    }
}
