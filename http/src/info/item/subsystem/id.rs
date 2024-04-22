#[serde_with::serde_as]
#[derive(serde::Serialize)]
pub(crate) struct HSubsystemInfoId {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub(crate) id: rc::SolItemId,
}
impl From<&rc::SolSubsystemInfo> for HSubsystemInfoId {
    fn from(core_subsystem_info: &rc::SolSubsystemInfo) -> Self {
        Self {
            id: core_subsystem_info.id,
        }
    }
}
