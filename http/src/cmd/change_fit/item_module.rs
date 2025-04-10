use crate::{
    cmd::{
        HCmdResp, change_item,
        shared::{HAddMode, HMutationOnAdd},
    },
    shared::{HModRack, HModuleState},
    util::HExecError,
};

#[derive(serde::Deserialize)]
pub(crate) struct HAddModuleCmd {
    rack: HModRack,
    add_mode: HAddMode,
    type_id: rc::ItemTypeId,
    state: HModuleState,
    mutation: Option<HMutationOnAdd>,
    charge_type_id: Option<rc::ItemTypeId>,
}
impl HAddModuleCmd {
    pub(in crate::cmd) fn execute(
        &self,
        core_sol: &mut rc::SolarSystem,
        fit_id: &rc::FitId,
    ) -> Result<rc::ModuleInfo, HExecError> {
        let core_module = match core_sol.add_module(
            fit_id,
            (&self.rack).into(),
            (&self.add_mode).into(),
            self.type_id,
            (&self.state).into(),
            self.mutation.as_ref().map(|v| v.into()),
            self.charge_type_id,
        ) {
            Ok(core_module) => core_module,
            Err(error) => {
                return Err(match error {
                    rc::err::AddModuleError::FitNotFound(e) => HExecError::FitNotFoundPrimary(e),
                });
            }
        };
        Ok(core_module)
    }
}

#[serde_with::serde_as]
#[derive(serde::Deserialize)]
pub(crate) struct HChangeModuleCmd {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    item_id: rc::ItemId,
    #[serde(flatten)]
    item_cmd: change_item::HChangeModuleCmd,
}
impl HChangeModuleCmd {
    pub(in crate::cmd) fn execute(&self, core_sol: &mut rc::SolarSystem) -> Result<HCmdResp, HExecError> {
        self.item_cmd.execute(core_sol, &self.item_id)
    }
}
