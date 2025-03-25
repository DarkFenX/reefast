#[serde_with::serde_as]
#[derive(serde::Serialize)]
pub(crate) struct HAutochargeInfoPartial {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub(crate) id: rc::ItemId,
    pub(crate) kind: &'static str,
    pub(crate) type_id: rc::ItemTypeId,
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub(crate) fit_id: rc::FitId,
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub(crate) cont_id: rc::ItemId,
    pub(crate) enabled: bool,
}
impl From<&rc::AutochargeInfo> for HAutochargeInfoPartial {
    fn from(core_autocharge_info: &rc::AutochargeInfo) -> Self {
        Self {
            id: core_autocharge_info.id,
            kind: "autocharge",
            type_id: core_autocharge_info.type_id,
            fit_id: core_autocharge_info.fit_id,
            cont_id: core_autocharge_info.cont_id,
            enabled: core_autocharge_info.enabled,
        }
    }
}
