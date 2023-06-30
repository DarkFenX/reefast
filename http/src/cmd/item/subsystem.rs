use crate::cmd::{
    shared::{apply_effect_modes, HEffectModeMap},
    HCmdResp,
};

#[derive(serde::Deserialize)]
pub(crate) struct HChangeSubsystemCmd {
    state: Option<bool>,
    effect_modes: Option<HEffectModeMap>,
}
impl HChangeSubsystemCmd {
    pub(in crate::cmd) fn execute(
        &self,
        core_ss: &mut rc::SolarSystem,
        item_id: &rc::SsItemId,
    ) -> rc::Result<HCmdResp> {
        if let Some(state) = self.state {
            core_ss.set_subsystem_state(item_id, state)?;
        }
        apply_effect_modes(core_ss, item_id, &self.effect_modes)?;
        Ok(HCmdResp::NoData)
    }
}
