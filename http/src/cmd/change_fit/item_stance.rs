use rc::ItemCommon;

use crate::{
    cmd::{HItemIdsResp, change_item, shared::get_primary_fit},
    util::HExecError,
};

#[derive(serde::Deserialize)]
pub(crate) struct HSetStanceCmd {
    type_id: rc::ItemTypeId,
    state: Option<bool>,
}
impl HSetStanceCmd {
    pub(in crate::cmd) fn execute(
        &self,
        core_sol: &mut rc::SolarSystem,
        fit_id: &rc::FitId,
    ) -> Result<HItemIdsResp, HExecError> {
        let mut core_fit = get_primary_fit(core_sol, fit_id)?;
        let mut core_stance = core_fit.set_stance(self.type_id);
        if let Some(state) = self.state {
            core_stance.set_state(state);
        }
        Ok(core_stance.into())
    }
}

#[derive(serde::Deserialize)]
#[serde(untagged)]
pub(crate) enum HChangeStanceCmd {
    ViaItemId(HChangeStanceViaItemIdCmd),
    ViaFitId(HChangeStanceViaFitIdCmd),
}
impl HChangeStanceCmd {
    pub(in crate::cmd) fn execute(
        &self,
        core_sol: &mut rc::SolarSystem,
        fit_id: &rc::FitId,
    ) -> Result<HItemIdsResp, HExecError> {
        match self {
            Self::ViaItemId(cmd) => cmd.execute(core_sol),
            Self::ViaFitId(cmd) => cmd.execute(core_sol, fit_id),
        }
    }
}

#[serde_with::serde_as]
#[derive(serde::Deserialize)]
pub(crate) struct HChangeStanceViaItemIdCmd {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    item_id: rc::ItemId,
    #[serde(flatten)]
    item_cmd: change_item::HChangeStanceCmd,
}
impl HChangeStanceViaItemIdCmd {
    pub(in crate::cmd) fn execute(&self, core_sol: &mut rc::SolarSystem) -> Result<HItemIdsResp, HExecError> {
        self.item_cmd.execute(core_sol, &self.item_id)
    }
}

#[derive(serde::Deserialize)]
pub(crate) struct HChangeStanceViaFitIdCmd {
    #[serde(flatten)]
    item_cmd: change_item::HChangeStanceCmd,
}
impl HChangeStanceViaFitIdCmd {
    pub(in crate::cmd) fn execute(
        &self,
        core_sol: &mut rc::SolarSystem,
        fit_id: &rc::FitId,
    ) -> Result<HItemIdsResp, HExecError> {
        let core_fit = get_primary_fit(core_sol, fit_id)?;
        let stance_item_id = match core_fit.get_stance() {
            Some(core_stance) => core_stance.get_item_id(),
            None => return Err(HExecError::FitStanceNotFound(*fit_id)),
        };
        self.item_cmd.execute(core_sol, &stance_item_id)
    }
}

#[derive(serde::Deserialize)]
pub(crate) struct HRemoveStanceCmd {}
impl HRemoveStanceCmd {
    pub(in crate::cmd) fn execute(&self, core_sol: &mut rc::SolarSystem, fit_id: &rc::FitId) -> Result<(), HExecError> {
        let mut core_fit = get_primary_fit(core_sol, fit_id)?;
        if let Some(core_stance) = core_fit.get_stance_mut() {
            core_stance.remove();
        }
        Ok(())
    }
}
