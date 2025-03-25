use crate::{
    cmd::{
        HCmdResp,
        shared::{HEffectModeMap, apply_effect_modes},
    },
    util::HExecError,
};

#[serde_with::serde_as]
#[derive(serde::Deserialize)]
pub(crate) struct HChangeChargeCmd {
    state: Option<bool>,
    effect_modes: Option<HEffectModeMap>,
}
impl HChangeChargeCmd {
    pub(in crate::cmd) fn execute(
        &self,
        core_sol: &mut rc::SolarSystem,
        item_id: &rc::ItemId,
    ) -> Result<HCmdResp, HExecError> {
        if let Some(state) = self.state {
            if let Err(error) = core_sol.set_charge_state(item_id, state) {
                return Err(match error {
                    rc::err::SetChargeStateError::ItemNotFound(e) => HExecError::ItemNotFoundPrimary(e),
                    rc::err::SetChargeStateError::ItemIsNotCharge(e) => HExecError::ItemKindMismatch(e),
                });
            }
        }
        apply_effect_modes(core_sol, item_id, &self.effect_modes)?;
        Ok(HCmdResp::NoData)
    }
}
