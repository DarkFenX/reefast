use rc::ItemCommon;

use crate::{
    cmd::{HItemIdsResp, change_item, shared::get_primary_fit},
    shared::{HCoordinates, HMovement},
    util::HExecError,
};

#[derive(serde::Deserialize)]
pub(crate) struct HSetShipCmd {
    type_id: rc::ItemTypeId,
    state: Option<bool>,
    coordinates: Option<HCoordinates>,
    movement: Option<HMovement>,
}
impl HSetShipCmd {
    pub(in crate::cmd) fn execute(
        &self,
        core_sol: &mut rc::SolarSystem,
        fit_id: &rc::FitId,
    ) -> Result<HItemIdsResp, HExecError> {
        let mut core_fit = get_primary_fit(core_sol, fit_id)?;
        let mut core_ship = core_fit.set_ship(
            self.type_id,
            self.coordinates.map(|v| v.into()),
            self.movement.map(|v| v.into()),
        );
        if let Some(state) = self.state {
            core_ship.set_state(state);
        }
        Ok(core_ship.into())
    }
}

#[derive(serde::Deserialize)]
#[serde(untagged)]
pub(crate) enum HChangeShipCmd {
    ViaItemId(HChangeShipViaItemIdCmd),
    ViaFitId(HChangeShipViaFitIdCmd),
}
impl HChangeShipCmd {
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
pub(crate) struct HChangeShipViaItemIdCmd {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    item_id: rc::ItemId,
    #[serde(flatten)]
    item_cmd: change_item::HChangeShipCmd,
}
impl HChangeShipViaItemIdCmd {
    pub(in crate::cmd) fn execute(&self, core_sol: &mut rc::SolarSystem) -> Result<HItemIdsResp, HExecError> {
        self.item_cmd.execute(core_sol, &self.item_id)
    }
}

#[derive(serde::Deserialize)]
pub(crate) struct HChangeShipViaFitIdCmd {
    #[serde(flatten)]
    item_cmd: change_item::HChangeShipCmd,
}
impl HChangeShipViaFitIdCmd {
    pub(in crate::cmd) fn execute(
        &self,
        core_sol: &mut rc::SolarSystem,
        fit_id: &rc::FitId,
    ) -> Result<HItemIdsResp, HExecError> {
        let core_fit = get_primary_fit(core_sol, fit_id)?;
        let ship_item_id = match core_fit.get_ship() {
            Some(core_ship) => core_ship.get_item_id(),
            None => return Err(HExecError::FitShipNotFound(*fit_id)),
        };
        self.item_cmd.execute(core_sol, &ship_item_id)
    }
}

#[derive(serde::Deserialize)]
pub(crate) struct HRemoveShipCmd {}
impl HRemoveShipCmd {
    pub(in crate::cmd) fn execute(&self, core_sol: &mut rc::SolarSystem, fit_id: &rc::FitId) -> Result<(), HExecError> {
        let mut core_fit = get_primary_fit(core_sol, fit_id)?;
        if let Some(core_ship) = core_fit.get_ship_mut() {
            core_ship.remove();
        }
        Ok(())
    }
}
