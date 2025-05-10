use std::collections::HashMap;

use crate::{
    ac,
    sol::{
        AttrVal, ItemId, ItemKey,
        svc::vast::VastFitData,
        uad::{Uad, fit::UadFit},
    },
    util::RSet,
};

pub struct ValChargeSizeFail {
    /// Map between charge IDs and info about failed validation.
    pub charges: HashMap<ItemId, ValChargeSizeChargeInfo>,
}

pub struct ValChargeSizeChargeInfo {
    /// Parent module item ID.
    pub parent_item_id: ItemId,
    /// Size attribute value of current charge.
    pub charge_size: Option<AttrVal>,
    /// Size value allowed by module.
    pub allowed_size: AttrVal,
}
impl ValChargeSizeChargeInfo {
    fn from_fail_data(uad: &Uad, fail_data: FailData) -> Self {
        Self {
            parent_item_id: uad.items.id_by_key(fail_data.parent_item_key),
            charge_size: fail_data.charge_size,
            allowed_size: fail_data.allowed_size,
        }
    }
}

struct FailData {
    parent_item_key: ItemKey,
    charge_item_key: ItemKey,
    charge_size: Option<AttrVal>,
    allowed_size: AttrVal,
}

impl VastFitData {
    // Fast validations
    pub(in crate::sol::svc::vast) fn validate_charge_size_fast(
        &mut self,
        kfs: &RSet<ItemKey>,
        uad: &Uad,
        fit: &UadFit,
    ) -> bool {
        iter_fails(kfs, uad, fit).next().is_none()
    }
    // Verbose validations
    pub(in crate::sol::svc::vast) fn validate_charge_size_verbose(
        &mut self,
        kfs: &RSet<ItemKey>,
        uad: &Uad,
        fit: &UadFit,
    ) -> Option<ValChargeSizeFail> {
        let mut charges = HashMap::new();
        for fail_data in iter_fails(kfs, uad, fit) {
            charges.insert(
                uad.items.id_by_key(fail_data.charge_item_key),
                ValChargeSizeChargeInfo::from_fail_data(uad, fail_data),
            );
        }
        match charges.is_empty() {
            true => None,
            false => Some(ValChargeSizeFail { charges }),
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
        let allowed_size = *uad_module.get_a_attrs()?.get(&ac::attrs::CHARGE_SIZE)?;
        let charge_size = uad
            .items
            .get(charge_item_key)
            .get_a_attrs()?
            .get(&ac::attrs::CHARGE_SIZE)
            .copied();
        if charge_size == Some(allowed_size) || kfs.contains(&charge_item_key) {
            return None;
        }
        Some(FailData {
            parent_item_key: module_item_key,
            charge_item_key,
            charge_size,
            allowed_size,
        })
    })
}
