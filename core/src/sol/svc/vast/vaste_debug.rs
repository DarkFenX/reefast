use crate::sol::{
    svc::debug::{check_fit, check_item},
    uad::SolUad,
    SolDebugResult,
};

use super::{SolVast, SolVastFitData};

impl SolVast {
    pub(in crate::sol::svc) fn debug_consistency_check(&self, uad: &SolUad) -> SolDebugResult {
        for (fit_id, fit_data) in self.fit_datas.iter() {
            check_fit(uad, fit_id)?;
            fit_data.debug_consistency_check(uad)?;
        }
        Ok(())
    }
}

impl SolVastFitData {
    pub(in crate::sol::svc) fn debug_consistency_check(&self, uad: &SolUad) -> SolDebugResult {
        for item_id in self.mods_online.iter() {
            check_item(uad, item_id)?;
        }
        for item_id in self.rigs_rigslot_calibration.keys() {
            check_item(uad, item_id)?;
        }
        for item_id in self.drones_volume.keys() {
            check_item(uad, item_id)?;
        }
        Ok(())
    }
}
