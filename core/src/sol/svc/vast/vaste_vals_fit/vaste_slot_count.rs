use crate::{
    ac, ad,
    sol::{
        Count, Idx, ItemId, ItemKey,
        svc::{calc::Calc, vast::VastFitData},
        uad::{
            Uad,
            fit::{Fit, ItemVec},
        },
    },
    util::{RMap, RSet},
};

use super::shared::get_max_slots;

pub struct ValSlotCountFail {
    /// How many slots are taken by all the relevant items.
    pub used: Count,
    /// How many slots available.
    pub max: Option<Count>,
    /// IDs of items which break the validation limits. For unordered containers - all items, for
    /// ordered containers - only those which go past limit.
    pub users: Vec<ItemId>,
}

impl VastFitData {
    // Fast validations
    pub(in crate::sol::svc::vast) fn validate_rig_slot_count_fast(
        &self,
        kfs: &RSet<ItemKey>,
        uad: &Uad,
        calc: &mut Calc,
        fit: &Fit,
    ) -> bool {
        validate_fast_unordered_set(kfs, uad, calc, fit.ship, &ac::attrs::UPGRADE_SLOTS_LEFT, &fit.rigs)
    }
    pub(in crate::sol::svc::vast) fn validate_service_slot_count_fast(
        &self,
        kfs: &RSet<ItemKey>,
        uad: &Uad,
        calc: &mut Calc,
        fit: &Fit,
    ) -> bool {
        validate_fast_unordered_set(kfs, uad, calc, fit.ship, &ac::attrs::SERVICE_SLOTS, &fit.services)
    }
    pub(in crate::sol::svc::vast) fn validate_subsystem_slot_count_fast(
        &self,
        kfs: &RSet<ItemKey>,
        uad: &Uad,
        calc: &mut Calc,
        fit: &Fit,
    ) -> bool {
        validate_fast_unordered_set(kfs, uad, calc, fit.ship, &ac::attrs::MAX_SUBSYSTEMS, &fit.subsystems)
    }
    pub(in crate::sol::svc::vast) fn validate_launched_drone_count_fast(
        &self,
        kfs: &RSet<ItemKey>,
        uad: &Uad,
        calc: &mut Calc,
        fit: &Fit,
    ) -> bool {
        validate_fast_unordered_map(
            kfs,
            uad,
            calc,
            fit.character,
            &ac::attrs::MAX_ACTIVE_DRONES,
            &self.drones_online_bandwidth,
        )
    }
    pub(in crate::sol::svc::vast) fn validate_launched_fighter_count_fast(
        &self,
        kfs: &RSet<ItemKey>,
        uad: &Uad,
        calc: &mut Calc,
        fit: &Fit,
    ) -> bool {
        validate_fast_unordered_set(kfs, uad, calc, fit.ship, &ac::attrs::FTR_TUBES, &self.fighters_online)
    }
    pub(in crate::sol::svc::vast) fn validate_launched_support_fighter_count_fast(
        &self,
        kfs: &RSet<ItemKey>,
        uad: &Uad,
        calc: &mut Calc,
        fit: &Fit,
    ) -> bool {
        validate_fast_unordered_set(
            kfs,
            uad,
            calc,
            fit.ship,
            &ac::attrs::FTR_SUPPORT_SLOTS,
            &self.support_fighters_online,
        )
    }
    pub(in crate::sol::svc::vast) fn validate_launched_light_fighter_count_fast(
        &self,
        kfs: &RSet<ItemKey>,
        uad: &Uad,
        calc: &mut Calc,
        fit: &Fit,
    ) -> bool {
        validate_fast_unordered_set(
            kfs,
            uad,
            calc,
            fit.ship,
            &ac::attrs::FTR_LIGHT_SLOTS,
            &self.light_fighters_online,
        )
    }
    pub(in crate::sol::svc::vast) fn validate_launched_heavy_fighter_count_fast(
        &self,
        kfs: &RSet<ItemKey>,
        uad: &Uad,
        calc: &mut Calc,
        fit: &Fit,
    ) -> bool {
        validate_fast_unordered_set(
            kfs,
            uad,
            calc,
            fit.ship,
            &ac::attrs::FTR_HEAVY_SLOTS,
            &self.heavy_fighters_online,
        )
    }
    pub(in crate::sol::svc::vast) fn validate_launched_standup_support_fighter_count_fast(
        &self,
        kfs: &RSet<ItemKey>,
        uad: &Uad,
        calc: &mut Calc,
        fit: &Fit,
    ) -> bool {
        validate_fast_unordered_set(
            kfs,
            uad,
            calc,
            fit.ship,
            &ac::attrs::FTR_STANDUP_SUPPORT_SLOTS,
            &self.standup_support_fighters_online,
        )
    }
    pub(in crate::sol::svc::vast) fn validate_launched_standup_light_fighter_count_fast(
        &self,
        kfs: &RSet<ItemKey>,
        uad: &Uad,
        calc: &mut Calc,
        fit: &Fit,
    ) -> bool {
        validate_fast_unordered_set(
            kfs,
            uad,
            calc,
            fit.ship,
            &ac::attrs::FTR_STANDUP_LIGHT_SLOTS,
            &self.standup_light_fighters_online,
        )
    }
    pub(in crate::sol::svc::vast) fn validate_launched_standup_heavy_fighter_count_fast(
        &self,
        kfs: &RSet<ItemKey>,
        uad: &Uad,
        calc: &mut Calc,
        fit: &Fit,
    ) -> bool {
        validate_fast_unordered_set(
            kfs,
            uad,
            calc,
            fit.ship,
            &ac::attrs::FTR_STANDUP_HEAVY_SLOTS,
            &self.standup_heavy_fighters_online,
        )
    }
    pub(in crate::sol::svc::vast) fn validate_turret_slot_count_fast(
        &self,
        kfs: &RSet<ItemKey>,
        uad: &Uad,
        calc: &mut Calc,
        fit: &Fit,
    ) -> bool {
        validate_fast_unordered_set(
            kfs,
            uad,
            calc,
            fit.ship,
            &ac::attrs::TURRET_SLOTS_LEFT,
            &self.mods_turret,
        )
    }
    pub(in crate::sol::svc::vast) fn validate_launcher_slot_count_fast(
        &self,
        kfs: &RSet<ItemKey>,
        uad: &Uad,
        calc: &mut Calc,
        fit: &Fit,
    ) -> bool {
        validate_fast_unordered_set(
            kfs,
            uad,
            calc,
            fit.ship,
            &ac::attrs::LAUNCHER_SLOTS_LEFT,
            &self.mods_launcher,
        )
    }
    pub(in crate::sol::svc::vast) fn validate_high_slot_count_fast(
        &self,
        kfs: &RSet<ItemKey>,
        uad: &Uad,
        calc: &mut Calc,
        fit: &Fit,
    ) -> bool {
        validate_fast_ordered(kfs, uad, calc, fit.ship, &ac::attrs::HI_SLOTS, &fit.mods_high)
    }
    pub(in crate::sol::svc::vast) fn validate_mid_slot_count_fast(
        &self,
        kfs: &RSet<ItemKey>,
        uad: &Uad,
        calc: &mut Calc,
        fit: &Fit,
    ) -> bool {
        validate_fast_ordered(kfs, uad, calc, fit.ship, &ac::attrs::MED_SLOTS, &fit.mods_mid)
    }
    pub(in crate::sol::svc::vast) fn validate_low_slot_count_fast(
        &self,
        kfs: &RSet<ItemKey>,
        uad: &Uad,
        calc: &mut Calc,
        fit: &Fit,
    ) -> bool {
        validate_fast_ordered(kfs, uad, calc, fit.ship, &ac::attrs::LOW_SLOTS, &fit.mods_low)
    }
    // Verbose validations
    pub(in crate::sol::svc::vast) fn validate_rig_slot_count_verbose(
        &self,
        kfs: &RSet<ItemKey>,
        uad: &Uad,
        calc: &mut Calc,
        fit: &Fit,
    ) -> Option<ValSlotCountFail> {
        validate_verbose_unordered_set(kfs, uad, calc, fit.ship, &ac::attrs::UPGRADE_SLOTS_LEFT, &fit.rigs)
    }
    pub(in crate::sol::svc::vast) fn validate_service_slot_count_verbose(
        &self,
        kfs: &RSet<ItemKey>,
        uad: &Uad,
        calc: &mut Calc,
        fit: &Fit,
    ) -> Option<ValSlotCountFail> {
        validate_verbose_unordered_set(kfs, uad, calc, fit.ship, &ac::attrs::SERVICE_SLOTS, &fit.services)
    }
    pub(in crate::sol::svc::vast) fn validate_subsystem_slot_count_verbose(
        &self,
        kfs: &RSet<ItemKey>,
        uad: &Uad,
        calc: &mut Calc,
        fit: &Fit,
    ) -> Option<ValSlotCountFail> {
        validate_verbose_unordered_set(kfs, uad, calc, fit.ship, &ac::attrs::MAX_SUBSYSTEMS, &fit.subsystems)
    }
    pub(in crate::sol::svc::vast) fn validate_launched_drone_count_verbose(
        &self,
        kfs: &RSet<ItemKey>,
        uad: &Uad,
        calc: &mut Calc,
        fit: &Fit,
    ) -> Option<ValSlotCountFail> {
        validate_verbose_unordered_map(
            kfs,
            uad,
            calc,
            fit.character,
            &ac::attrs::MAX_ACTIVE_DRONES,
            &self.drones_online_bandwidth,
        )
    }
    pub(in crate::sol::svc::vast) fn validate_launched_fighter_count_verbose(
        &self,
        kfs: &RSet<ItemKey>,
        uad: &Uad,
        calc: &mut Calc,
        fit: &Fit,
    ) -> Option<ValSlotCountFail> {
        validate_verbose_unordered_set(kfs, uad, calc, fit.ship, &ac::attrs::FTR_TUBES, &self.fighters_online)
    }
    pub(in crate::sol::svc::vast) fn validate_launched_support_fighter_count_verbose(
        &self,
        kfs: &RSet<ItemKey>,
        uad: &Uad,
        calc: &mut Calc,
        fit: &Fit,
    ) -> Option<ValSlotCountFail> {
        validate_verbose_unordered_set(
            kfs,
            uad,
            calc,
            fit.ship,
            &ac::attrs::FTR_SUPPORT_SLOTS,
            &self.support_fighters_online,
        )
    }
    pub(in crate::sol::svc::vast) fn validate_launched_light_fighter_count_verbose(
        &self,
        kfs: &RSet<ItemKey>,
        uad: &Uad,
        calc: &mut Calc,
        fit: &Fit,
    ) -> Option<ValSlotCountFail> {
        validate_verbose_unordered_set(
            kfs,
            uad,
            calc,
            fit.ship,
            &ac::attrs::FTR_LIGHT_SLOTS,
            &self.light_fighters_online,
        )
    }
    pub(in crate::sol::svc::vast) fn validate_launched_heavy_fighter_count_verbose(
        &self,
        kfs: &RSet<ItemKey>,
        uad: &Uad,
        calc: &mut Calc,
        fit: &Fit,
    ) -> Option<ValSlotCountFail> {
        validate_verbose_unordered_set(
            kfs,
            uad,
            calc,
            fit.ship,
            &ac::attrs::FTR_HEAVY_SLOTS,
            &self.heavy_fighters_online,
        )
    }
    pub(in crate::sol::svc::vast) fn validate_launched_standup_support_fighter_count_verbose(
        &self,
        kfs: &RSet<ItemKey>,
        uad: &Uad,
        calc: &mut Calc,
        fit: &Fit,
    ) -> Option<ValSlotCountFail> {
        validate_verbose_unordered_set(
            kfs,
            uad,
            calc,
            fit.ship,
            &ac::attrs::FTR_STANDUP_SUPPORT_SLOTS,
            &self.standup_support_fighters_online,
        )
    }
    pub(in crate::sol::svc::vast) fn validate_launched_standup_light_fighter_count_verbose(
        &self,
        kfs: &RSet<ItemKey>,
        uad: &Uad,
        calc: &mut Calc,
        fit: &Fit,
    ) -> Option<ValSlotCountFail> {
        validate_verbose_unordered_set(
            kfs,
            uad,
            calc,
            fit.ship,
            &ac::attrs::FTR_STANDUP_LIGHT_SLOTS,
            &self.standup_light_fighters_online,
        )
    }
    pub(in crate::sol::svc::vast) fn validate_launched_standup_heavy_fighter_count_verbose(
        &self,
        kfs: &RSet<ItemKey>,
        uad: &Uad,
        calc: &mut Calc,
        fit: &Fit,
    ) -> Option<ValSlotCountFail> {
        validate_verbose_unordered_set(
            kfs,
            uad,
            calc,
            fit.ship,
            &ac::attrs::FTR_STANDUP_HEAVY_SLOTS,
            &self.standup_heavy_fighters_online,
        )
    }
    pub(in crate::sol::svc::vast) fn validate_turret_slot_count_verbose(
        &self,
        kfs: &RSet<ItemKey>,
        uad: &Uad,
        calc: &mut Calc,
        fit: &Fit,
    ) -> Option<ValSlotCountFail> {
        validate_verbose_unordered_set(
            kfs,
            uad,
            calc,
            fit.ship,
            &ac::attrs::TURRET_SLOTS_LEFT,
            &self.mods_turret,
        )
    }
    pub(in crate::sol::svc::vast) fn validate_launcher_slot_count_verbose(
        &self,
        kfs: &RSet<ItemKey>,
        uad: &Uad,
        calc: &mut Calc,
        fit: &Fit,
    ) -> Option<ValSlotCountFail> {
        validate_verbose_unordered_set(
            kfs,
            uad,
            calc,
            fit.ship,
            &ac::attrs::LAUNCHER_SLOTS_LEFT,
            &self.mods_launcher,
        )
    }
    pub(in crate::sol::svc::vast) fn validate_high_slot_count_verbose(
        &self,
        kfs: &RSet<ItemKey>,
        uad: &Uad,
        calc: &mut Calc,
        fit: &Fit,
    ) -> Option<ValSlotCountFail> {
        validate_verbose_ordered(kfs, uad, calc, fit.ship, &ac::attrs::HI_SLOTS, &fit.mods_high)
    }
    pub(in crate::sol::svc::vast) fn validate_mid_slot_count_verbose(
        &self,
        kfs: &RSet<ItemKey>,
        uad: &Uad,
        calc: &mut Calc,
        fit: &Fit,
    ) -> Option<ValSlotCountFail> {
        validate_verbose_ordered(kfs, uad, calc, fit.ship, &ac::attrs::MED_SLOTS, &fit.mods_mid)
    }
    pub(in crate::sol::svc::vast) fn validate_low_slot_count_verbose(
        &self,
        kfs: &RSet<ItemKey>,
        uad: &Uad,
        calc: &mut Calc,
        fit: &Fit,
    ) -> Option<ValSlotCountFail> {
        validate_verbose_ordered(kfs, uad, calc, fit.ship, &ac::attrs::LOW_SLOTS, &fit.mods_low)
    }
}

fn validate_fast_unordered_set(
    kfs: &RSet<ItemKey>,
    uad: &Uad,
    calc: &mut Calc,
    max_item_key: Option<ItemKey>,
    max_a_attr_id: &ad::AAttrId,
    users: &RSet<ItemKey>,
) -> bool {
    if users.is_subset(kfs) {
        return true;
    }
    let used = users.len() as Count;
    let max = get_max_slots(uad, calc, max_item_key, max_a_attr_id).unwrap_or(0);
    used <= max
}
fn validate_fast_unordered_map<T>(
    kfs: &RSet<ItemKey>,
    uad: &Uad,
    calc: &mut Calc,
    max_item_key: Option<ItemKey>,
    max_a_attr_id: &ad::AAttrId,
    users: &RMap<ItemKey, T>,
) -> bool {
    if users.is_subset(kfs) {
        return true;
    }
    let used = users.len() as Count;
    let max = get_max_slots(uad, calc, max_item_key, max_a_attr_id).unwrap_or(0);
    used <= max
}
fn validate_fast_ordered(
    kfs: &RSet<ItemKey>,
    uad: &Uad,
    calc: &mut Calc,
    max_item_key: Option<ItemKey>,
    max_a_attr_id: &ad::AAttrId,
    users: &ItemVec,
) -> bool {
    let used = users.len() as Count;
    let max = get_max_slots(uad, calc, max_item_key, max_a_attr_id).unwrap_or(0);
    match kfs.is_empty() {
        true => used <= max,
        false => match used <= max {
            true => true,
            false => users.iter_keys_from(max as Idx).all(|v| kfs.contains(v)),
        },
    }
}

fn validate_verbose_unordered_set(
    kfs: &RSet<ItemKey>,
    uad: &Uad,
    calc: &mut Calc,
    max_item_key: Option<ItemKey>,
    max_a_attr_id: &ad::AAttrId,
    users: &RSet<ItemKey>,
) -> Option<ValSlotCountFail> {
    let used = users.len() as Count;
    let max = get_max_slots(uad, calc, max_item_key, max_a_attr_id);
    if used <= max.unwrap_or(0) {
        return None;
    }
    let users: Vec<_> = users
        .difference(kfs)
        .map(|item_key| uad.items.id_by_key(*item_key))
        .collect();
    match users.is_empty() {
        true => None,
        false => Some(ValSlotCountFail { used, max, users }),
    }
}
fn validate_verbose_unordered_map<T>(
    kfs: &RSet<ItemKey>,
    uad: &Uad,
    calc: &mut Calc,
    max_item_key: Option<ItemKey>,
    max_a_attr_id: &ad::AAttrId,
    users: &RMap<ItemKey, T>,
) -> Option<ValSlotCountFail> {
    let used = users.len() as Count;
    let max = get_max_slots(uad, calc, max_item_key, max_a_attr_id);
    if used <= max.unwrap_or(0) {
        return None;
    }
    let users: Vec<_> = users
        .difference(kfs)
        .map(|(item_key, _)| uad.items.id_by_key(*item_key))
        .collect();
    match users.is_empty() {
        true => None,
        false => Some(ValSlotCountFail { used, max, users }),
    }
}
fn validate_verbose_ordered(
    kfs: &RSet<ItemKey>,
    uad: &Uad,
    calc: &mut Calc,
    max_item_key: Option<ItemKey>,
    max_a_attr_id: &ad::AAttrId,
    users: &ItemVec,
) -> Option<ValSlotCountFail> {
    let used = users.len() as Count;
    let max = get_max_slots(uad, calc, max_item_key, max_a_attr_id);
    let effective_max = max.unwrap_or(0);
    if used <= effective_max {
        return None;
    }
    let users: Vec<_> = users
        .iter_keys_from(effective_max as Idx)
        .filter(|item_key| !kfs.contains(item_key))
        .map(|item_key| uad.items.id_by_key(*item_key))
        .collect();
    match users.is_empty() {
        true => None,
        false => Some(ValSlotCountFail { used, max, users }),
    }
}
