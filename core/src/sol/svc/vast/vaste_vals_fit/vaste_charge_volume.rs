use std::collections::HashMap;

use ordered_float::OrderedFloat as OF;

use crate::{
    ac,
    sol::{
        AttrVal, ItemId, ItemKey,
        svc::vast::VastFitData,
        uad::{Uad, fit::UadFit},
    },
    util::RSet,
};

pub struct ValChargeVolumeFail {
    /// Map between charge IDs and info about failed validation.
    pub charges: HashMap<ItemId, ValChargeVolumeChargeInfo>,
}

pub struct ValChargeVolumeChargeInfo {
    /// Parent module item ID.
    pub parent_item_id: ItemId,
    /// Volume of current charge.
    pub charge_volume: AttrVal,
    /// Maximum charge volume allowed by its parent module.
    pub max_volume: AttrVal,
}
impl ValChargeVolumeChargeInfo {
    fn from_fail_data(uad: &Uad, fail_data: FailData) -> Self {
        Self {
            parent_item_id: uad.items.id_by_key(fail_data.parent_item_key),
            charge_volume: fail_data.charge_volume,
            max_volume: fail_data.max_volume,
        }
    }
}

struct FailData {
    parent_item_key: ItemKey,
    charge_item_key: ItemKey,
    charge_volume: AttrVal,
    max_volume: AttrVal,
}

impl VastFitData {
    // Fast validations
    pub(in crate::sol::svc::vast) fn validate_charge_volume_fast(
        &mut self,
        kfs: &RSet<ItemKey>,
        uad: &Uad,
        fit: &UadFit,
    ) -> bool {
        iter_fails(kfs, uad, fit).next().is_none()
    }
    // Verbose validations
    pub(in crate::sol::svc::vast) fn validate_charge_volume_verbose(
        &mut self,
        kfs: &RSet<ItemKey>,
        uad: &Uad,
        fit: &UadFit,
    ) -> Option<ValChargeVolumeFail> {
        let mut charges = HashMap::new();
        for fail_data in iter_fails(kfs, uad, fit) {
            charges.insert(
                uad.items.id_by_key(fail_data.charge_item_key),
                ValChargeVolumeChargeInfo::from_fail_data(uad, fail_data),
            );
        }
        match charges.is_empty() {
            true => None,
            false => Some(ValChargeVolumeFail { charges }),
        }
    }
}

fn iter_fails(kfs: &RSet<ItemKey>, uad: &Uad, fit: &UadFit) -> impl Iterator<Item = FailData> {
    itertools::chain!(
        fit.mods_high.iter_keys().copied(),
        fit.mods_mid.iter_keys().copied(),
        fit.mods_low.iter_keys().copied(),
    )
    .filter_map(|module_item_key| {
        let uad_module = uad.items.get(module_item_key).get_module().unwrap();
        let charge_item_key = uad_module.get_charge_item_key()?;
        let charge_volume = uad.items.get(charge_item_key).get_a_extras()?.volume?;
        let max_volume = match uad_module.get_a_attrs()?.get(&ac::attrs::CAPACITY) {
            Some(max_volume) => *max_volume,
            None => OF(0.0),
        };
        if charge_volume <= max_volume || kfs.contains(&charge_item_key) {
            return None;
        }
        Some(FailData {
            parent_item_key: module_item_key,
            charge_item_key,
            charge_volume,
            max_volume,
        })
    })
}
