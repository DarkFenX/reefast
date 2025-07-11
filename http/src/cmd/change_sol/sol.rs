use crate::shared::{HDpsProfile, HSecZone};

#[derive(serde::Deserialize)]
pub(crate) struct HChangeSolCmd {
    sec_zone: Option<HSecZone>,
    default_incoming_dps: Option<HDpsProfile>,
}
impl HChangeSolCmd {
    pub(in crate::cmd) fn execute(&self, core_sol: &mut rc::SolarSystem) {
        if let Some(sec_zone) = &self.sec_zone {
            core_sol.set_sec_zone(sec_zone.into());
        }
        if let Some(default_incoming_dps) = self.default_incoming_dps {
            core_sol.set_default_incoming_dps(default_incoming_dps.into());
        }
    }
}
