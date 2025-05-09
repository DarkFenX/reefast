use crate::{
    cmd::{change_fit, shared::HItemIdsResp},
    util::HExecError,
};

#[serde_with::serde_as]
#[derive(serde::Deserialize)]
pub(crate) struct HAddModuleCmd {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    fit_id: rc::FitId,
    #[serde(flatten)]
    fit_cmd: change_fit::HAddModuleCmd,
}
impl HAddModuleCmd {
    pub(in crate::cmd) fn execute(&self, core_sol: &mut rc::SolarSystem) -> Result<HItemIdsResp, HExecError> {
        self.fit_cmd.execute(core_sol, &self.fit_id)
    }
}

#[derive(serde::Deserialize)]
pub(crate) struct HChangeModuleCmd {
    #[serde(flatten)]
    fit_cmd: change_fit::HChangeModuleCmd,
}
impl HChangeModuleCmd {
    pub(in crate::cmd) fn execute(&self, core_sol: &mut rc::SolarSystem) -> Result<HItemIdsResp, HExecError> {
        self.fit_cmd.execute(core_sol)
    }
}
