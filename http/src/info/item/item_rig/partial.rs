use rc::ItemCommon;

#[serde_with::serde_as]
#[derive(serde::Serialize)]
pub(crate) struct HRigInfoPartial {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    id: rc::ItemId,
    kind: &'static str,
    type_id: rc::ItemTypeId,
    #[serde_as(as = "serde_with::DisplayFromStr")]
    fit_id: rc::FitId,
    enabled: bool,
}
impl From<&mut rc::RigMut<'_>> for HRigInfoPartial {
    fn from(core_rig: &mut rc::RigMut) -> Self {
        Self {
            id: core_rig.get_item_id(),
            kind: "rig",
            type_id: core_rig.get_type_id(),
            fit_id: core_rig.get_fit().get_fit_id(),
            enabled: core_rig.get_state(),
        }
    }
}
