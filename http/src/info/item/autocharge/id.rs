#[serde_with::serde_as]
#[derive(serde::Serialize)]
pub(crate) struct HAutochargeInfoId {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub(crate) id: rc::SolItemId,
}
impl From<&rc::SolAutochargeInfo> for HAutochargeInfoId {
    fn from(core_autocharge_info: &rc::SolAutochargeInfo) -> Self {
        Self {
            id: core_autocharge_info.id,
        }
    }
}
