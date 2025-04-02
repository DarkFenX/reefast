use crate::{
    shared::{HDpsProfile, HSecZone},
    util::HExecError,
};

#[derive(Default, serde::Deserialize)]
pub(crate) struct HAddSolCmd {
    sec_zone: Option<HSecZone>,
    default_incoming_dps: Option<HDpsProfile>,
}
impl HAddSolCmd {
    pub(crate) fn execute(&self, src: rc::Src) -> Result<rc::SolarSystem, HExecError> {
        let mut core_sol = rc::SolarSystem::new(src);
        if let Some(sec_zone) = &self.sec_zone {
            core_sol.set_sec_zone(sec_zone.into());
        }
        if let Some(default_incoming_dps) = &self.default_incoming_dps {
            core_sol.set_default_incoming_dps(default_incoming_dps.try_into()?);
        }
        Ok(core_sol)
    }
}
