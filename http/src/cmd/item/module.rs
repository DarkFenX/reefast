use crate::{
    cmd::{
        shared::{apply_effect_modes, HEffectModeMap},
        HCmdResp,
    },
    shared::HState,
};

#[serde_with::serde_as]
#[derive(serde::Deserialize)]
pub(crate) struct HChangeModuleCmd {
    state: Option<HState>,
    #[serde(default, with = "::serde_with::rust::double_option")]
    charge: Option<Option<rc::EItemId>>,
    // Workaround for https://github.com/serde-rs/serde/issues/1183
    #[serde_as(as = "Option<std::collections::HashMap<serde_with::DisplayFromStr, _>>")]
    effect_modes: Option<HEffectModeMap>,
}
impl HChangeModuleCmd {
    pub(in crate::cmd) fn execute(
        &self,
        core_ss: &mut rc::SolarSystem,
        item_id: &rc::SsItemId,
    ) -> rc::Result<HCmdResp> {
        if let Some(state) = &self.state {
            core_ss.set_module_state(item_id, state.into())?;
        }
        if let Some(charge_opt) = &self.charge {
            match charge_opt {
                Some(charge) => {
                    core_ss.set_module_charge(item_id, *charge)?;
                }
                None => {
                    core_ss.remove_module_charge(item_id)?;
                }
            }
        }
        apply_effect_modes(core_ss, item_id, &self.effect_modes)?;
        Ok(HCmdResp::NoData)
    }
}
