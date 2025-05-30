use crate::{
    cmd::{HItemIdsResp, change_fit},
    util::HExecError,
};

#[derive(serde::Deserialize)]
pub(crate) struct HChangeChargeCmd {
    #[serde(flatten)]
    fit_cmd: change_fit::HChangeChargeCmd,
}
impl HChangeChargeCmd {
    pub(in crate::cmd) fn execute(&self, core_sol: &mut rc::SolarSystem) -> Result<HItemIdsResp, HExecError> {
        self.fit_cmd.execute(core_sol)
    }
}
