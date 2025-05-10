use crate::{
    ac, ad,
    sol::{
        Count, Idx, ItemId, ItemKey,
        svc::{calc::Calc, vast::VastFitData},
        uad::{
            Uad,
            fit::{ItemVec, UadFit},
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
        fit: &UadFit,
    ) -> bool {
        validate_fast_unordered(
            kfs,
            uad,
            calc,
            fit.ship,
            &ac::attrs::UPGRADE_SLOTS_LEFT,
            fit.rigs.iter().copied(),
        )
    }
    pub(in crate::sol::svc::vast) fn validate_service_slot_count_fast(
        &self,
        kfs: &RSet<ItemKey>,
        uad: &Uad,
        calc: &mut Calc,
        fit: &UadFit,
    ) -> bool {
        validate_fast_unordered(
            kfs,
            uad,
            calc,
            fit.ship,
            &ac::attrs::SERVICE_SLOTS,
            fit.services.iter().copied(),
        )
    }
    pub(in crate::sol::svc::vast) fn validate_subsystem_slot_count_fast(
        &self,
        kfs: &RSet<ItemKey>,
        uad: &Uad,
        calc: &mut Calc,
        fit: &UadFit,
    ) -> bool {
        validate_fast_unordered(
            kfs,
            uad,
            calc,
            fit.ship,
            &ac::attrs::MAX_SUBSYSTEMS,
            fit.subsystems.iter().copied(),
        )
    }
    pub(in crate::sol::svc::vast) fn validate_launched_drone_count_fast(
        &self,
        kfs: &RSet<ItemKey>,
        uad: &Uad,
        calc: &mut Calc,
        fit: &UadFit,
    ) -> bool {
        validate_fast_unordered(
            kfs,
            uad,
            calc,
            fit.character,
            &ac::attrs::MAX_ACTIVE_DRONES,
            iter_online_drones(uad, fit),
        )
    }
    pub(in crate::sol::svc::vast) fn validate_launched_fighter_count_fast(
        &self,
        kfs: &RSet<ItemKey>,
        uad: &Uad,
        calc: &mut Calc,
        fit: &UadFit,
    ) -> bool {
        validate_fast_unordered(
            kfs,
            uad,
            calc,
            fit.ship,
            &ac::attrs::FTR_TUBES,
            iter_online_fighters(uad, fit),
        )
    }
    pub(in crate::sol::svc::vast) fn validate_launched_support_fighter_count_fast(
        &self,
        kfs: &RSet<ItemKey>,
        uad: &Uad,
        calc: &mut Calc,
        fit: &UadFit,
    ) -> bool {
        validate_fast_unordered(
            kfs,
            uad,
            calc,
            fit.ship,
            &ac::attrs::FTR_SUPPORT_SLOTS,
            iter_online_support_fighters(uad, fit),
        )
    }
    pub(in crate::sol::svc::vast) fn validate_launched_light_fighter_count_fast(
        &self,
        kfs: &RSet<ItemKey>,
        uad: &Uad,
        calc: &mut Calc,
        fit: &UadFit,
    ) -> bool {
        validate_fast_unordered(
            kfs,
            uad,
            calc,
            fit.ship,
            &ac::attrs::FTR_LIGHT_SLOTS,
            iter_online_light_fighters(uad, fit),
        )
    }
    pub(in crate::sol::svc::vast) fn validate_launched_heavy_fighter_count_fast(
        &self,
        kfs: &RSet<ItemKey>,
        uad: &Uad,
        calc: &mut Calc,
        fit: &UadFit,
    ) -> bool {
        validate_fast_unordered(
            kfs,
            uad,
            calc,
            fit.ship,
            &ac::attrs::FTR_HEAVY_SLOTS,
            iter_online_heavy_fighters(uad, fit),
        )
    }
    pub(in crate::sol::svc::vast) fn validate_launched_standup_support_fighter_count_fast(
        &self,
        kfs: &RSet<ItemKey>,
        uad: &Uad,
        calc: &mut Calc,
        fit: &UadFit,
    ) -> bool {
        validate_fast_unordered(
            kfs,
            uad,
            calc,
            fit.ship,
            &ac::attrs::FTR_STANDUP_SUPPORT_SLOTS,
            iter_online_standup_support_fighters(uad, fit),
        )
    }
    pub(in crate::sol::svc::vast) fn validate_launched_standup_light_fighter_count_fast(
        &self,
        kfs: &RSet<ItemKey>,
        uad: &Uad,
        calc: &mut Calc,
        fit: &UadFit,
    ) -> bool {
        validate_fast_unordered(
            kfs,
            uad,
            calc,
            fit.ship,
            &ac::attrs::FTR_STANDUP_LIGHT_SLOTS,
            iter_online_standup_light_fighters(uad, fit),
        )
    }
    pub(in crate::sol::svc::vast) fn validate_launched_standup_heavy_fighter_count_fast(
        &self,
        kfs: &RSet<ItemKey>,
        uad: &Uad,
        calc: &mut Calc,
        fit: &UadFit,
    ) -> bool {
        validate_fast_unordered(
            kfs,
            uad,
            calc,
            fit.ship,
            &ac::attrs::FTR_STANDUP_HEAVY_SLOTS,
            iter_online_standup_heavy_fighters(uad, fit),
        )
    }
    pub(in crate::sol::svc::vast) fn validate_turret_slot_count_fast(
        &self,
        kfs: &RSet<ItemKey>,
        uad: &Uad,
        calc: &mut Calc,
        fit: &UadFit,
    ) -> bool {
        validate_fast_unordered(
            kfs,
            uad,
            calc,
            fit.ship,
            &ac::attrs::TURRET_SLOTS_LEFT,
            iter_turrets(uad, fit),
        )
    }
    pub(in crate::sol::svc::vast) fn validate_launcher_slot_count_fast(
        &self,
        kfs: &RSet<ItemKey>,
        uad: &Uad,
        calc: &mut Calc,
        fit: &UadFit,
    ) -> bool {
        validate_fast_unordered(
            kfs,
            uad,
            calc,
            fit.ship,
            &ac::attrs::LAUNCHER_SLOTS_LEFT,
            iter_launchers(uad, fit),
        )
    }
    pub(in crate::sol::svc::vast) fn validate_high_slot_count_fast(
        &self,
        kfs: &RSet<ItemKey>,
        uad: &Uad,
        calc: &mut Calc,
        fit: &UadFit,
    ) -> bool {
        validate_fast_ordered(kfs, uad, calc, fit.ship, &ac::attrs::HI_SLOTS, &fit.mods_high)
    }
    pub(in crate::sol::svc::vast) fn validate_mid_slot_count_fast(
        &self,
        kfs: &RSet<ItemKey>,
        uad: &Uad,
        calc: &mut Calc,
        fit: &UadFit,
    ) -> bool {
        validate_fast_ordered(kfs, uad, calc, fit.ship, &ac::attrs::MED_SLOTS, &fit.mods_mid)
    }
    pub(in crate::sol::svc::vast) fn validate_low_slot_count_fast(
        &self,
        kfs: &RSet<ItemKey>,
        uad: &Uad,
        calc: &mut Calc,
        fit: &UadFit,
    ) -> bool {
        validate_fast_ordered(kfs, uad, calc, fit.ship, &ac::attrs::LOW_SLOTS, &fit.mods_low)
    }
    // Verbose validations
    pub(in crate::sol::svc::vast) fn validate_rig_slot_count_verbose(
        &self,
        kfs: &RSet<ItemKey>,
        uad: &Uad,
        calc: &mut Calc,
        fit: &UadFit,
    ) -> Option<ValSlotCountFail> {
        validate_verbose_unordered(
            kfs,
            uad,
            calc,
            fit.ship,
            &ac::attrs::UPGRADE_SLOTS_LEFT,
            fit.rigs.iter().copied(),
        )
    }
    pub(in crate::sol::svc::vast) fn validate_service_slot_count_verbose(
        &self,
        kfs: &RSet<ItemKey>,
        uad: &Uad,
        calc: &mut Calc,
        fit: &UadFit,
    ) -> Option<ValSlotCountFail> {
        validate_verbose_unordered(
            kfs,
            uad,
            calc,
            fit.ship,
            &ac::attrs::SERVICE_SLOTS,
            fit.services.iter().copied(),
        )
    }
    pub(in crate::sol::svc::vast) fn validate_subsystem_slot_count_verbose(
        &self,
        kfs: &RSet<ItemKey>,
        uad: &Uad,
        calc: &mut Calc,
        fit: &UadFit,
    ) -> Option<ValSlotCountFail> {
        validate_verbose_unordered(
            kfs,
            uad,
            calc,
            fit.ship,
            &ac::attrs::MAX_SUBSYSTEMS,
            fit.subsystems.iter().copied(),
        )
    }
    pub(in crate::sol::svc::vast) fn validate_launched_drone_count_verbose(
        &self,
        kfs: &RSet<ItemKey>,
        uad: &Uad,
        calc: &mut Calc,
        fit: &UadFit,
    ) -> Option<ValSlotCountFail> {
        validate_verbose_unordered(
            kfs,
            uad,
            calc,
            fit.character,
            &ac::attrs::MAX_ACTIVE_DRONES,
            iter_online_drones(uad, fit),
        )
    }
    pub(in crate::sol::svc::vast) fn validate_launched_fighter_count_verbose(
        &self,
        kfs: &RSet<ItemKey>,
        uad: &Uad,
        calc: &mut Calc,
        fit: &UadFit,
    ) -> Option<ValSlotCountFail> {
        validate_verbose_unordered(
            kfs,
            uad,
            calc,
            fit.ship,
            &ac::attrs::FTR_TUBES,
            iter_online_fighters(uad, fit),
        )
    }
    pub(in crate::sol::svc::vast) fn validate_launched_support_fighter_count_verbose(
        &self,
        kfs: &RSet<ItemKey>,
        uad: &Uad,
        calc: &mut Calc,
        fit: &UadFit,
    ) -> Option<ValSlotCountFail> {
        validate_verbose_unordered(
            kfs,
            uad,
            calc,
            fit.ship,
            &ac::attrs::FTR_SUPPORT_SLOTS,
            iter_online_support_fighters(uad, fit),
        )
    }
    pub(in crate::sol::svc::vast) fn validate_launched_light_fighter_count_verbose(
        &self,
        kfs: &RSet<ItemKey>,
        uad: &Uad,
        calc: &mut Calc,
        fit: &UadFit,
    ) -> Option<ValSlotCountFail> {
        validate_verbose_unordered(
            kfs,
            uad,
            calc,
            fit.ship,
            &ac::attrs::FTR_LIGHT_SLOTS,
            iter_online_light_fighters(uad, fit),
        )
    }
    pub(in crate::sol::svc::vast) fn validate_launched_heavy_fighter_count_verbose(
        &self,
        kfs: &RSet<ItemKey>,
        uad: &Uad,
        calc: &mut Calc,
        fit: &UadFit,
    ) -> Option<ValSlotCountFail> {
        validate_verbose_unordered(
            kfs,
            uad,
            calc,
            fit.ship,
            &ac::attrs::FTR_HEAVY_SLOTS,
            iter_online_heavy_fighters(uad, fit),
        )
    }
    pub(in crate::sol::svc::vast) fn validate_launched_standup_support_fighter_count_verbose(
        &self,
        kfs: &RSet<ItemKey>,
        uad: &Uad,
        calc: &mut Calc,
        fit: &UadFit,
    ) -> Option<ValSlotCountFail> {
        validate_verbose_unordered(
            kfs,
            uad,
            calc,
            fit.ship,
            &ac::attrs::FTR_STANDUP_SUPPORT_SLOTS,
            iter_online_standup_support_fighters(uad, fit),
        )
    }
    pub(in crate::sol::svc::vast) fn validate_launched_standup_light_fighter_count_verbose(
        &self,
        kfs: &RSet<ItemKey>,
        uad: &Uad,
        calc: &mut Calc,
        fit: &UadFit,
    ) -> Option<ValSlotCountFail> {
        validate_verbose_unordered(
            kfs,
            uad,
            calc,
            fit.ship,
            &ac::attrs::FTR_STANDUP_LIGHT_SLOTS,
            iter_online_standup_light_fighters(uad, fit),
        )
    }
    pub(in crate::sol::svc::vast) fn validate_launched_standup_heavy_fighter_count_verbose(
        &self,
        kfs: &RSet<ItemKey>,
        uad: &Uad,
        calc: &mut Calc,
        fit: &UadFit,
    ) -> Option<ValSlotCountFail> {
        validate_verbose_unordered(
            kfs,
            uad,
            calc,
            fit.ship,
            &ac::attrs::FTR_STANDUP_HEAVY_SLOTS,
            iter_online_standup_heavy_fighters(uad, fit),
        )
    }
    pub(in crate::sol::svc::vast) fn validate_turret_slot_count_verbose(
        &self,
        kfs: &RSet<ItemKey>,
        uad: &Uad,
        calc: &mut Calc,
        fit: &UadFit,
    ) -> Option<ValSlotCountFail> {
        validate_verbose_unordered(
            kfs,
            uad,
            calc,
            fit.ship,
            &ac::attrs::TURRET_SLOTS_LEFT,
            iter_turrets(uad, fit),
        )
    }
    pub(in crate::sol::svc::vast) fn validate_launcher_slot_count_verbose(
        &self,
        kfs: &RSet<ItemKey>,
        uad: &Uad,
        calc: &mut Calc,
        fit: &UadFit,
    ) -> Option<ValSlotCountFail> {
        validate_verbose_unordered(
            kfs,
            uad,
            calc,
            fit.ship,
            &ac::attrs::LAUNCHER_SLOTS_LEFT,
            iter_launchers(uad, fit),
        )
    }
    pub(in crate::sol::svc::vast) fn validate_high_slot_count_verbose(
        &self,
        kfs: &RSet<ItemKey>,
        uad: &Uad,
        calc: &mut Calc,
        fit: &UadFit,
    ) -> Option<ValSlotCountFail> {
        validate_verbose_ordered(kfs, uad, calc, fit.ship, &ac::attrs::HI_SLOTS, &fit.mods_high)
    }
    pub(in crate::sol::svc::vast) fn validate_mid_slot_count_verbose(
        &self,
        kfs: &RSet<ItemKey>,
        uad: &Uad,
        calc: &mut Calc,
        fit: &UadFit,
    ) -> Option<ValSlotCountFail> {
        validate_verbose_ordered(kfs, uad, calc, fit.ship, &ac::attrs::MED_SLOTS, &fit.mods_mid)
    }
    pub(in crate::sol::svc::vast) fn validate_low_slot_count_verbose(
        &self,
        kfs: &RSet<ItemKey>,
        uad: &Uad,
        calc: &mut Calc,
        fit: &UadFit,
    ) -> Option<ValSlotCountFail> {
        validate_verbose_ordered(kfs, uad, calc, fit.ship, &ac::attrs::LOW_SLOTS, &fit.mods_low)
    }
}

fn validate_fast_unordered(
    kfs: &RSet<ItemKey>,
    uad: &Uad,
    calc: &mut Calc,
    max_item_key: Option<ItemKey>,
    max_a_attr_id: &ad::AAttrId,
    user_iter: impl Iterator<Item = ItemKey>,
) -> bool {
    let mut used = 0;
    let mut force_pass = true;
    for user in user_iter {
        if force_pass && !kfs.contains(&user) {
            force_pass = false;
        }
        used += 1;
    }
    if force_pass {
        return true;
    }
    let max = get_max_slots(uad, calc, max_item_key, max_a_attr_id).unwrap_or(0);
    used <= max
}
fn validate_fast_ordered(
    kfs: &RSet<ItemKey>,
    uad: &Uad,
    calc: &mut Calc,
    max_item_key: Option<ItemKey>,
    max_a_attr_id: &ad::AAttrId,
    user_vec: &ItemVec,
) -> bool {
    let used = user_vec.len() as Count;
    let max = get_max_slots(uad, calc, max_item_key, max_a_attr_id).unwrap_or(0);
    match kfs.is_empty() {
        true => used <= max,
        false => match used <= max {
            true => true,
            false => user_vec.iter_keys_from(max as Idx).all(|v| kfs.contains(v)),
        },
    }
}

fn validate_verbose_unordered(
    kfs: &RSet<ItemKey>,
    uad: &Uad,
    calc: &mut Calc,
    max_item_key: Option<ItemKey>,
    max_a_attr_id: &ad::AAttrId,
    user_iter: impl Iterator<Item = ItemKey>,
) -> Option<ValSlotCountFail> {
    let mut used = 0;
    let mut users = Vec::new();
    for item_key in user_iter {
        used += 1;
        if !kfs.contains(&item_key) {
            users.push(uad.items.id_by_key(item_key));
        }
    }
    if users.is_empty() {
        return None;
    }
    let max = get_max_slots(uad, calc, max_item_key, max_a_attr_id);
    if used <= max.unwrap_or(0) {
        return None;
    }
    Some(ValSlotCountFail { used, max, users })
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

fn iter_online_drones(uad: &Uad, fit: &UadFit) -> impl Iterator<Item = ItemKey> {
    fit.drones
        .iter()
        .copied()
        .filter(|item_key| uad.items.get(*item_key).get_a_state() >= ad::AState::Online)
}

fn iter_online_fighters(uad: &Uad, fit: &UadFit) -> impl Iterator<Item = ItemKey> {
    fit.fighters
        .iter()
        .copied()
        .filter(|item_key| uad.items.get(*item_key).get_a_state() >= ad::AState::Online)
}

fn iter_online_support_fighters(uad: &Uad, fit: &UadFit) -> impl Iterator<Item = ItemKey> {
    fit.fighters.iter().copied().filter(|item_key| {
        let uad_fighter = uad.items.get(*item_key).get_fighter().unwrap();
        uad_fighter.get_a_state() >= ad::AState::Online
            && match uad_fighter.get_a_extras() {
                Some(a_extras) => a_extras.is_support_fighter,
                None => false,
            }
    })
}

fn iter_online_light_fighters(uad: &Uad, fit: &UadFit) -> impl Iterator<Item = ItemKey> {
    fit.fighters.iter().copied().filter(|item_key| {
        let uad_fighter = uad.items.get(*item_key).get_fighter().unwrap();
        uad_fighter.get_a_state() >= ad::AState::Online
            && match uad_fighter.get_a_extras() {
                Some(a_extras) => a_extras.is_light_fighter,
                None => false,
            }
    })
}

fn iter_online_heavy_fighters(uad: &Uad, fit: &UadFit) -> impl Iterator<Item = ItemKey> {
    fit.fighters.iter().copied().filter(|item_key| {
        let uad_fighter = uad.items.get(*item_key).get_fighter().unwrap();
        uad_fighter.get_a_state() >= ad::AState::Online
            && match uad_fighter.get_a_extras() {
                Some(a_extras) => a_extras.is_heavy_fighter,
                None => false,
            }
    })
}

fn iter_online_standup_support_fighters(uad: &Uad, fit: &UadFit) -> impl Iterator<Item = ItemKey> {
    fit.fighters.iter().copied().filter(|item_key| {
        let uad_fighter = uad.items.get(*item_key).get_fighter().unwrap();
        uad_fighter.get_a_state() >= ad::AState::Online
            && match uad_fighter.get_a_extras() {
                Some(a_extras) => a_extras.is_standup_support_fighter,
                None => false,
            }
    })
}

fn iter_online_standup_light_fighters(uad: &Uad, fit: &UadFit) -> impl Iterator<Item = ItemKey> {
    fit.fighters.iter().copied().filter(|item_key| {
        let uad_fighter = uad.items.get(*item_key).get_fighter().unwrap();
        uad_fighter.get_a_state() >= ad::AState::Online
            && match uad_fighter.get_a_extras() {
                Some(a_extras) => a_extras.is_standup_light_fighter,
                None => false,
            }
    })
}

fn iter_online_standup_heavy_fighters(uad: &Uad, fit: &UadFit) -> impl Iterator<Item = ItemKey> {
    fit.fighters.iter().copied().filter(|item_key| {
        let uad_fighter = uad.items.get(*item_key).get_fighter().unwrap();
        uad_fighter.get_a_state() >= ad::AState::Online
            && match uad_fighter.get_a_extras() {
                Some(a_extras) => a_extras.is_standup_heavy_fighter,
                None => false,
            }
    })
}

fn iter_turrets(uad: &Uad, fit: &UadFit) -> impl Iterator<Item = ItemKey> {
    itertools::chain!(
        fit.mods_high.iter_keys().copied(),
        fit.mods_mid.iter_keys().copied(),
        fit.mods_low.iter_keys().copied(),
    )
    .filter(|item_key| match uad.items.get(*item_key).get_a_extras() {
        Some(a_extras) => a_extras.takes_turret_hardpoint,
        None => false,
    })
}

fn iter_launchers(uad: &Uad, fit: &UadFit) -> impl Iterator<Item = ItemKey> {
    itertools::chain!(
        fit.mods_high.iter_keys().copied(),
        fit.mods_mid.iter_keys().copied(),
        fit.mods_low.iter_keys().copied(),
    )
    .filter(|item_key| match uad.items.get(*item_key).get_a_extras() {
        Some(a_extras) => a_extras.takes_launcher_hardpoint,
        None => false,
    })
}
