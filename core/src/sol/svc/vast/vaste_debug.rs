use crate::sol::{
    debug::{DebugResult, check_fit_key, check_item_key},
    uad::Uad,
};

use super::{Vast, VastFitData};

impl Vast {
    pub(in crate::sol::svc) fn consistency_check(&self, uad: &Uad) -> DebugResult {
        for (&fit_key, fit_data) in self.fit_datas.iter() {
            check_fit_key(uad, fit_key)?;
            fit_data.consistency_check(uad)?;
        }
        Ok(())
    }
}

impl VastFitData {
    pub(in crate::sol::svc) fn consistency_check(&self, uad: &Uad) -> DebugResult {
        for &item_key in self.drones_bandwidth.keys() {
            check_item_key(uad, item_key, true)?;
        }
        for &item_key in self.support_fighters.iter() {
            check_item_key(uad, item_key, true)?;
        }
        for &item_key in self.light_fighters.iter() {
            check_item_key(uad, item_key, true)?;
        }
        for &item_key in self.heavy_fighters.iter() {
            check_item_key(uad, item_key, true)?;
        }
        for &item_key in self.standup_support_fighters.iter() {
            check_item_key(uad, item_key, true)?;
        }
        for &item_key in self.standup_light_fighters.iter() {
            check_item_key(uad, item_key, true)?;
        }
        for &item_key in self.standup_heavy_fighters.iter() {
            check_item_key(uad, item_key, true)?;
        }
        for item_keys in self.slotted_implants.values() {
            for &item_key in item_keys {
                check_item_key(uad, item_key, true)?;
            }
        }
        for item_keys in self.slotted_boosters.values() {
            for &item_key in item_keys {
                check_item_key(uad, item_key, true)?;
            }
        }
        for item_keys in self.slotted_subsystems.values() {
            for &item_key in item_keys {
                check_item_key(uad, item_key, true)?;
            }
        }
        for &item_key in self.ship_limited_items.keys() {
            check_item_key(uad, item_key, true)?;
        }
        for item_keys in self.mods_svcs_rigs_max_group_fitted_all.values() {
            for &item_key in item_keys {
                check_item_key(uad, item_key, true)?;
            }
        }
        for &item_key in self.mods_svcs_rigs_max_group_fitted_limited.keys() {
            check_item_key(uad, item_key, true)?;
        }
        for item_keys in self.mods_svcs_max_group_online_all.values() {
            for &item_key in item_keys {
                check_item_key(uad, item_key, true)?;
            }
        }
        for &item_key in self.mods_svcs_max_group_online_limited.keys() {
            check_item_key(uad, item_key, true)?;
        }
        for item_keys in self.mods_max_group_active_all.values() {
            for &item_key in item_keys {
                check_item_key(uad, item_key, true)?;
            }
        }
        for &item_key in self.mods_max_group_active_limited.keys() {
            check_item_key(uad, item_key, true)?;
        }
        for &item_key in self.rigs_rig_size.keys() {
            // This container can store info about non-loaded rigs
            check_item_key(uad, item_key, false)?;
        }
        for item_keys in self.srqs_skill_item_map.values() {
            for &item_key in item_keys {
                check_item_key(uad, item_key, true)?;
            }
        }
        for &item_key in self.srqs_missing.keys() {
            check_item_key(uad, item_key, true)?;
        }
        for &item_key in self.not_loaded.iter() {
            check_item_key(uad, item_key, false)?;
        }
        for &item_key in self.mods_state.keys() {
            check_item_key(uad, item_key, true)?;
        }
        for &item_key in self.item_kind.keys() {
            check_item_key(uad, item_key, true)?;
        }
        for &item_key in self.drone_groups.keys() {
            check_item_key(uad, item_key, true)?;
        }
        for &item_key in self.fighter_squad_size.keys() {
            check_item_key(uad, item_key, true)?;
        }
        for &item_key in self.overload_td_lvl.keys() {
            check_item_key(uad, item_key, true)?;
        }
        for item_data in self.mods_svcs_max_type_fitted.values() {
            for &item_key in item_data.keys() {
                check_item_key(uad, item_key, true)?;
            }
        }
        for &item_key in self.sec_zone_fitted.iter() {
            check_item_key(uad, item_key, true)?;
        }
        for &item_key in self.sec_zone_fitted_wspace_banned.iter() {
            check_item_key(uad, item_key, true)?;
        }
        for &item_key in self.sec_zone_online_class.keys() {
            check_item_key(uad, item_key, true)?;
        }
        for &item_key in self.sec_zone_active.iter() {
            check_item_key(uad, item_key, true)?;
        }
        for &item_key in self.sec_zone_unonlineable_class.keys() {
            check_item_key(uad, item_key, true)?;
        }
        for &item_key in self.sec_zone_unactivable.iter() {
            check_item_key(uad, item_key, true)?;
        }
        for &item_key in self.mods_rigs_svcs_vs_ship_kind.keys() {
            check_item_key(uad, item_key, true)?;
        }
        Ok(())
    }
}
