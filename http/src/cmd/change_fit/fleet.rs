use crate::{cmd::HCmdResp, util::HExecResult};

#[serde_with::serde_as]
#[derive(serde::Deserialize)]
pub(crate) struct HSetFleetCmd {
    #[serde_as(as = "Option<serde_with::DisplayFromStr>")]
    fleet_id: Option<rc::SolFleetId>,
}
impl HSetFleetCmd {
    pub(in crate::cmd) fn execute(
        &self,
        core_sol: &mut rc::SolarSystem,
        fit_id: &rc::SolFitId,
    ) -> HExecResult<HCmdResp> {
        core_sol.set_fit_fleet(fit_id, self.fleet_id)?;
        Ok(HCmdResp::NoData)
    }
}
