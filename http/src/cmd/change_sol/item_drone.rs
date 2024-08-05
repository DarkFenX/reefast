use crate::{
    cmd::{change_fit, HCmdResp},
    util::HExecResult,
};

#[serde_with::serde_as]
#[derive(serde::Deserialize)]
pub(crate) struct HAddDroneCmd {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    fit_id: rc::SolFitId,
    #[serde(flatten)]
    fit_cmd: change_fit::HAddDroneCmd,
}
impl HAddDroneCmd {
    pub(in crate::cmd) fn execute(&self, core_sol: &mut rc::SolarSystem) -> HExecResult<rc::SolDroneInfo> {
        self.fit_cmd.execute(core_sol, &self.fit_id)
    }
}

#[derive(serde::Deserialize)]
pub(crate) struct HChangeDroneCmd {
    #[serde(flatten)]
    fit_cmd: change_fit::HChangeDroneCmd,
}
impl HChangeDroneCmd {
    pub(in crate::cmd) fn execute(&self, core_sol: &mut rc::SolarSystem) -> HExecResult<HCmdResp> {
        self.fit_cmd.execute(core_sol)
    }
}
