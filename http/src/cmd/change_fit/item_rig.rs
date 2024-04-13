use crate::cmd::{change_item, HCmdResp};

#[derive(serde::Deserialize)]
pub(crate) struct HAddRigCmd {
    type_id: rc::EItemId,
    state: Option<bool>,
}
impl HAddRigCmd {
    pub(in crate::cmd) fn execute(
        &self,
        core_ss: &mut rc::SolarSystem,
        fit_id: &rc::SsFitId,
    ) -> rc::Result<rc::SsRigInfo> {
        core_ss.add_rig(*fit_id, self.type_id, self.state.unwrap_or(true))
    }
}

#[serde_with::serde_as]
#[derive(serde::Deserialize)]
pub(crate) struct HChangeRigCmd {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    item_id: rc::SsItemId,
    #[serde(flatten)]
    item_cmd: change_item::HChangeRigCmd,
}
impl HChangeRigCmd {
    pub(in crate::cmd) fn execute(&self, core_ss: &mut rc::SolarSystem) -> rc::Result<HCmdResp> {
        self.item_cmd.execute(core_ss, &self.item_id)
    }
}
