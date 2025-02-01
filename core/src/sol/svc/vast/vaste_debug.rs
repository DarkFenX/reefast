use crate::sol::{
    debug::{check_fit, check_item, SolDebugResult},
    svc::vast::SolValCache,
    uad::SolUad,
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
            check_item(uad, item_id, true)?;
        }
        for item_id in self.rigs_rigslot_calibration.keys() {
            check_item(uad, item_id, true)?;
        }
        for item_id in self.drones_volume.keys() {
            check_item(uad, item_id, true)?;
        }
        for item_id in self.drones_online_bandwidth.keys() {
            // Holds not loaded drones as well
            check_item(uad, item_id, false)?;
        }
        for item_id in self.fighters_online.iter() {
            check_item(uad, item_id, false)?;
        }
        for item_id in self.support_fighters_online.iter() {
            check_item(uad, item_id, true)?;
        }
        for item_id in self.light_fighters_online.iter() {
            check_item(uad, item_id, true)?;
        }
        for item_id in self.heavy_fighters_online.iter() {
            check_item(uad, item_id, true)?;
        }
        for item_id in self.standup_support_fighters_online.iter() {
            check_item(uad, item_id, true)?;
        }
        for item_id in self.standup_light_fighters_online.iter() {
            check_item(uad, item_id, true)?;
        }
        for item_id in self.standup_heavy_fighters_online.iter() {
            check_item(uad, item_id, true)?;
        }
        for item_id in self.mods_turret.iter() {
            check_item(uad, item_id, true)?;
        }
        for item_id in self.mods_launcher.iter() {
            check_item(uad, item_id, true)?;
        }
        for item_ids in self.slotted_implants.values() {
            for item_id in item_ids {
                check_item(uad, item_id, true)?;
            }
        }
        for item_ids in self.slotted_boosters.values() {
            for item_id in item_ids {
                check_item(uad, item_id, true)?;
            }
        }
        for item_ids in self.slotted_subsystems.values() {
            for item_id in item_ids {
                check_item(uad, item_id, true)?;
            }
        }
        for item_id in self.ship_limited_mods_rigs_subs.keys() {
            check_item(uad, item_id, true)?;
        }
        for item_ids in self.mods_rigs_max_group_fitted_all.values() {
            for item_id in item_ids {
                check_item(uad, item_id, true)?;
            }
        }
        for item_id in self.mods_rigs_max_group_fitted_limited.keys() {
            check_item(uad, item_id, true)?;
        }
        for item_ids in self.mods_max_group_online_all.values() {
            for item_id in item_ids {
                check_item(uad, item_id, true)?;
            }
        }
        for item_id in self.mods_max_group_online_limited.keys() {
            check_item(uad, item_id, true)?;
        }
        for item_ids in self.mods_max_group_active_all.values() {
            for item_id in item_ids {
                check_item(uad, item_id, true)?;
            }
        }
        for item_id in self.mods_max_group_active_limited.keys() {
            check_item(uad, item_id, true)?;
        }
        for item_id in self.rigs_rig_size.keys() {
            // This container can store info about non-loaded rigs
            check_item(uad, item_id, false)?;
        }
        for item_ids in self.srqs_skill_item_map.values() {
            for item_id in item_ids {
                check_item(uad, item_id, true)?;
            }
        }
        for item_id in self.srqs_missing.keys() {
            check_item(uad, item_id, true)?;
        }
        for (item_id, item_data) in self.mods_charge_group.iter() {
            check_item(uad, item_id, true)?;
            if let SolValCache::Fail(item_fail) = item_data {
                check_item(uad, &item_fail.parent_item_id, true)?;
                // This container can store info about non-loaded charges
                check_item(uad, &item_fail.charge_item_id, false)?;
            }
        }
        for (item_id, item_data) in self.mods_charge_size.iter() {
            check_item(uad, item_id, true)?;
            if let SolValCache::Fail(item_fail) = item_data {
                check_item(uad, &item_fail.parent_item_id, true)?;
                // This container can store info about non-loaded charges
                check_item(uad, &item_fail.charge_item_id, false)?;
            }
        }
        for (item_id, item_data) in self.mods_charge_volume.iter() {
            // This container can store info about non-loaded modules
            check_item(uad, item_id, false)?;
            if let SolValCache::Fail(item_fail) = item_data {
                // This container can store info about non-loaded modules
                check_item(uad, &item_fail.parent_item_id, false)?;
                check_item(uad, &item_fail.charge_item_id, true)?;
            }
        }
        Ok(())
    }
}
