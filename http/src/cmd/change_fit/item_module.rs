use crate::{
    cmd::{change_item, shared::HAddMode, HCmdResp},
    shared::{HModRack, HState},
    util::HExecError,
};

#[derive(serde::Deserialize)]
pub(crate) struct HAddModuleCmd {
    rack: HModRack,
    add_mode: HAddMode,
    type_id: rc::EItemId,
    state: HState,
    charge_type_id: Option<rc::EItemId>,
}
impl HAddModuleCmd {
    pub(in crate::cmd) fn execute(
        &self,
        core_sol: &mut rc::SolarSystem,
        fit_id: &rc::SolFitId,
    ) -> Result<rc::SolModuleInfo, HExecError> {
        let core_module = match core_sol.add_module(
            *fit_id,
            (&self.rack).into(),
            (&self.add_mode).into(),
            self.type_id,
            (&self.state).into(),
            None,
            self.charge_type_id,
        ) {
            Ok(core_module) => core_module,
            Err(error) => {
                return Err(match error {
                    rc::err::AddModuleError::FitNotFound(e) => HExecError::FitNotFoundPrimary(e),
                    rc::err::AddModuleError::ItemIdAllocFailed(e) => HExecError::ItemCapacityReached(e),
                    rc::err::AddModuleError::SlotTaken(e) => HExecError::ModuleSlotTaken(e),
                })
            }
        };
        Ok(core_module)
    }
}

#[serde_with::serde_as]
#[derive(serde::Deserialize)]
pub(crate) struct HChangeModuleCmd {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    item_id: rc::SolItemId,
    #[serde(flatten)]
    item_cmd: change_item::HChangeModuleCmd,
}
impl HChangeModuleCmd {
    pub(in crate::cmd) fn execute(&self, core_sol: &mut rc::SolarSystem) -> Result<HCmdResp, HExecError> {
        self.item_cmd.execute(core_sol, &self.item_id)
    }
}
