use ordered_float::OrderedFloat as OF;
use std::collections::HashMap;

use crate::{
    ac, ad,
    sol::{
        AttrVal, ItemId, ItemKey,
        svc::{calc::Calc, vast::VastFitData},
        uad::{Uad, fit::UadFit},
    },
    util::{RSet, round},
};

use super::shared::get_max_resource;

pub struct ValResFail {
    /// How much resource is used by all of its consumers.
    pub used: AttrVal,
    /// Max available resource (e.g. amount of CPU produced by ship).
    pub max: Option<AttrVal>,
    /// Map between consumer item IDs and amount consumed.
    pub users: HashMap<ItemId, AttrVal>,
}

impl VastFitData {
    // Fast validations
    pub(in crate::sol::svc::vast) fn validate_cpu_fast(
        &self,
        kfs: &RSet<ItemKey>,
        uad: &Uad,
        calc: &mut Calc,
        fit: &UadFit,
    ) -> bool {
        validate_fast_fitting(kfs, uad, calc, fit, &ac::attrs::CPU, &ac::attrs::CPU_OUTPUT)
    }
    pub(in crate::sol::svc::vast) fn validate_powergrid_fast(
        &self,
        kfs: &RSet<ItemKey>,
        uad: &Uad,
        calc: &mut Calc,
        fit: &UadFit,
    ) -> bool {
        validate_fast_fitting(kfs, uad, calc, fit, &ac::attrs::POWER, &ac::attrs::POWER_OUTPUT)
    }
    pub(in crate::sol::svc::vast) fn validate_calibration_fast(
        &self,
        kfs: &RSet<ItemKey>,
        uad: &Uad,
        calc: &mut Calc,
        fit: &UadFit,
    ) -> bool {
        validate_fast_other(
            kfs,
            uad,
            calc,
            fit,
            iter_rigs_offline_calibration(uad, fit),
            &ac::attrs::UPGRADE_CAPACITY,
        )
    }
    pub(in crate::sol::svc::vast) fn validate_drone_bay_volume_fast(
        &self,
        kfs: &RSet<ItemKey>,
        uad: &Uad,
        calc: &mut Calc,
        fit: &UadFit,
    ) -> bool {
        validate_fast_other(
            kfs,
            uad,
            calc,
            fit,
            iter_drones_volume(uad, fit),
            &ac::attrs::DRONE_CAPACITY,
        )
    }
    pub(in crate::sol::svc::vast) fn validate_drone_bandwidth_fast(
        &self,
        kfs: &RSet<ItemKey>,
        uad: &Uad,
        calc: &mut Calc,
        fit: &UadFit,
    ) -> bool {
        validate_fast_other(
            kfs,
            uad,
            calc,
            fit,
            iter_drones_online_bandwidth(uad, fit),
            &ac::attrs::DRONE_BANDWIDTH,
        )
    }
    pub(in crate::sol::svc::vast) fn validate_fighter_bay_volume_fast(
        &self,
        kfs: &RSet<ItemKey>,
        uad: &Uad,
        calc: &mut Calc,
        fit: &UadFit,
    ) -> bool {
        validate_fast_other(
            kfs,
            uad,
            calc,
            fit,
            iter_fighters_volume(uad, fit),
            &ac::attrs::FTR_CAPACITY,
        )
    }
    // Verbose validations
    pub(in crate::sol::svc::vast) fn validate_cpu_verbose(
        &self,
        kfs: &RSet<ItemKey>,
        uad: &Uad,
        calc: &mut Calc,
        fit: &UadFit,
    ) -> Option<ValResFail> {
        validate_verbose_fitting(kfs, uad, calc, fit, &ac::attrs::CPU, &ac::attrs::CPU_OUTPUT)
    }
    pub(in crate::sol::svc::vast) fn validate_powergrid_verbose(
        &self,
        kfs: &RSet<ItemKey>,
        uad: &Uad,
        calc: &mut Calc,
        fit: &UadFit,
    ) -> Option<ValResFail> {
        validate_verbose_fitting(kfs, uad, calc, fit, &ac::attrs::POWER, &ac::attrs::POWER_OUTPUT)
    }
    pub(in crate::sol::svc::vast) fn validate_calibration_verbose(
        &self,
        kfs: &RSet<ItemKey>,
        uad: &Uad,
        calc: &mut Calc,
        fit: &UadFit,
    ) -> Option<ValResFail> {
        validate_verbose_other(
            kfs,
            uad,
            calc,
            fit,
            iter_rigs_offline_calibration(uad, fit),
            &ac::attrs::UPGRADE_CAPACITY,
        )
    }
    pub(in crate::sol::svc::vast) fn validate_drone_bay_volume_verbose(
        &self,
        kfs: &RSet<ItemKey>,
        uad: &Uad,
        calc: &mut Calc,
        fit: &UadFit,
    ) -> Option<ValResFail> {
        validate_verbose_other(
            kfs,
            uad,
            calc,
            fit,
            iter_drones_volume(uad, fit),
            &ac::attrs::DRONE_CAPACITY,
        )
    }
    pub(in crate::sol::svc::vast) fn validate_drone_bandwidth_verbose(
        &self,
        kfs: &RSet<ItemKey>,
        uad: &Uad,
        calc: &mut Calc,
        fit: &UadFit,
    ) -> Option<ValResFail> {
        validate_verbose_other(
            kfs,
            uad,
            calc,
            fit,
            iter_drones_online_bandwidth(uad, fit),
            &ac::attrs::DRONE_BANDWIDTH,
        )
    }
    pub(in crate::sol::svc::vast) fn validate_fighter_bay_volume_verbose(
        &self,
        kfs: &RSet<ItemKey>,
        uad: &Uad,
        calc: &mut Calc,
        fit: &UadFit,
    ) -> Option<ValResFail> {
        validate_verbose_other(
            kfs,
            uad,
            calc,
            fit,
            iter_fighters_volume(uad, fit),
            &ac::attrs::FTR_CAPACITY,
        )
    }
}

fn validate_fast_fitting<'a>(
    kfs: &RSet<ItemKey>,
    uad: &Uad,
    calc: &mut Calc,
    fit: &UadFit,
    use_a_attr_id: &ad::AAttrId,
    max_a_attr_id: &ad::AAttrId,
) -> bool {
    let mut total_use = OF(0.0);
    let mut force_pass = true;
    for item_key in iter_fitting_users(uad, fit) {
        let item_use = match calc.get_item_attr_val_extra(uad, item_key, use_a_attr_id) {
            Some(item_use) => item_use,
            None => continue,
        };
        if force_pass && item_use > OF(0.0) && !kfs.contains(&item_key) {
            force_pass = false;
        }
        total_use += item_use;
    }
    if force_pass {
        return true;
    }
    let max = get_max_resource(uad, calc, fit.ship, max_a_attr_id).unwrap_or(OF(0.0));
    round(total_use, 2) <= max
}
fn validate_fast_other<'a>(
    kfs: &RSet<ItemKey>,
    uad: &Uad,
    calc: &mut Calc,
    fit: &UadFit,
    items: impl Iterator<Item = (ItemKey, ad::AAttrVal)>,
    max_a_attr_id: &ad::AAttrId,
) -> bool {
    let mut total_use = OF(0.0);
    let mut force_pass = true;
    for (item_key, item_use) in items {
        if force_pass && item_use > OF(0.0) && !kfs.contains(&item_key) {
            force_pass = false;
        }
        total_use += item_use;
    }
    if force_pass {
        return true;
    }
    let max = get_max_resource(uad, calc, fit.ship, max_a_attr_id).unwrap_or(OF(0.0));
    total_use <= max
}

fn validate_verbose_fitting<'a>(
    kfs: &RSet<ItemKey>,
    uad: &Uad,
    calc: &mut Calc,
    fit: &UadFit,
    use_a_attr_id: &ad::AAttrId,
    max_a_attr_id: &ad::AAttrId,
) -> Option<ValResFail> {
    let mut total_use = OF(0.0);
    let mut users = HashMap::new();
    for item_key in iter_fitting_users(uad, fit) {
        let item_use = match calc.get_item_attr_val_extra(uad, item_key, use_a_attr_id) {
            Some(item_use) => item_use,
            None => continue,
        };
        total_use += item_use;
        if item_use > OF(0.0) && !kfs.contains(&item_key) {
            users.insert(uad.items.id_by_key(item_key), item_use);
        }
    }
    if users.is_empty() {
        return None;
    }
    let total_use = round(total_use, 2);
    let max = get_max_resource(uad, calc, fit.ship, max_a_attr_id);
    if total_use <= max.unwrap_or(OF(0.0)) {
        return None;
    }
    Some(ValResFail {
        used: total_use,
        max,
        users,
    })
}
fn validate_verbose_other<'a>(
    kfs: &RSet<ItemKey>,
    uad: &Uad,
    calc: &mut Calc,
    fit: &UadFit,
    items: impl Iterator<Item = (ItemKey, ad::AAttrVal)>,
    max_a_attr_id: &ad::AAttrId,
) -> Option<ValResFail> {
    let mut total_use = OF(0.0);
    let mut users = HashMap::new();
    for (item_key, item_use) in items {
        total_use += item_use;
        if item_use > OF(0.0) && !kfs.contains(&item_key) {
            users.insert(uad.items.id_by_key(item_key), item_use);
        }
    }
    if users.is_empty() {
        return None;
    }
    let max = get_max_resource(uad, calc, fit.ship, max_a_attr_id);
    if total_use <= max.unwrap_or(OF(0.0)) {
        return None;
    }
    Some(ValResFail {
        used: total_use,
        max,
        users,
    })
}

fn iter_fitting_users(uad: &Uad, fit: &UadFit) -> impl Iterator<Item = ItemKey> {
    itertools::chain!(
        fit.mods_high.iter_keys().copied(),
        fit.mods_mid.iter_keys().copied(),
        fit.mods_low.iter_keys().copied(),
        fit.services.iter().copied(),
    )
    .filter(|item_key| uad.items.get(*item_key).get_a_state() >= ad::AState::Online)
}

fn iter_rigs_offline_calibration(uad: &Uad, fit: &UadFit) -> impl Iterator<Item = (ItemKey, AttrVal)> {
    fit.rigs.iter().copied().filter_map(|item_key| {
        let uad_rig = uad.items.get(item_key).get_rig().unwrap();
        match uad_rig.get_a_state() < ad::AState::Offline {
            true => None,
            false => uad_rig
                .get_a_attrs()
                .and_then(|a_attrs| a_attrs.get(&ac::attrs::UPGRADE_COST).map(|val| (item_key, *val))),
        }
    })
}

fn iter_drones_volume(uad: &Uad, fit: &UadFit) -> impl Iterator<Item = (ItemKey, AttrVal)> {
    fit.drones.iter().copied().filter_map(|item_key| {
        let uad_drone = uad.items.get(item_key).get_drone().unwrap();
        uad_drone
            .get_a_extras()
            .and_then(|a_extras| a_extras.volume)
            .map(|volume| (item_key, volume))
    })
}

fn iter_drones_online_bandwidth(uad: &Uad, fit: &UadFit) -> impl Iterator<Item = (ItemKey, AttrVal)> {
    fit.drones.iter().copied().filter_map(|item_key| {
        let uad_drone = uad.items.get(item_key).get_drone().unwrap();
        match uad_drone.get_a_state() < ad::AState::Online {
            true => None,
            false => uad_drone
                .get_a_extras()
                .and_then(|a_extras| a_extras.bandwidth_use)
                .map(|bandwidth| (item_key, bandwidth)),
        }
    })
}

fn iter_fighters_volume(uad: &Uad, fit: &UadFit) -> impl Iterator<Item = (ItemKey, AttrVal)> {
    fit.fighters.iter().copied().filter_map(|item_key| {
        let uad_fighter = uad.items.get(item_key).get_fighter().unwrap();
        uad_fighter
            .get_a_extras()
            .and_then(|a_extras| a_extras.volume)
            .map(|volume| {
                (
                    item_key,
                    volume * AttrVal::from(uad_fighter.get_count().unwrap().current),
                )
            })
    })
}
