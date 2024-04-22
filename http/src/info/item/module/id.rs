#[serde_with::serde_as]
#[derive(serde::Serialize)]
pub(crate) struct HModuleInfoId {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub(crate) id: rc::SolItemId,
    #[serde_as(as = "Option<serde_with::DisplayFromStr>")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) charge_id: Option<rc::SolItemId>,
}
impl From<&rc::SolModuleInfo> for HModuleInfoId {
    fn from(core_module_info: &rc::SolModuleInfo) -> Self {
        Self {
            id: core_module_info.id,
            charge_id: core_module_info.charge_info.as_ref().map(|v| v.id),
        }
    }
}
