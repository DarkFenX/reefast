use std::collections::HashMap;

use crate::{
    sol::{
        ItemGrpId, ItemId, ItemKey,
        svc::vast::VastFitData,
        uad::{Uad, fit::UadFit},
    },
    util::RSet,
};

pub struct ValChargeGroupFail {
    /// Map between charge IDs and info about failed validation.
    pub charges: HashMap<ItemId, ValChargeGroupChargeInfo>,
}

pub struct ValChargeGroupChargeInfo {
    /// Parent module item ID.
    pub parent_item_id: ItemId,
    /// Group ID of current charge.
    pub charge_group_id: ItemGrpId,
    /// Group IDs allowed by containing module.
    pub allowed_group_ids: Vec<ItemGrpId>,
}
impl ValChargeGroupChargeInfo {
    fn from_fail_data(uad: &Uad, fail_data: FailData) -> Self {
        Self {
            parent_item_id: uad.items.id_by_key(fail_data.parent_item_key),
            charge_group_id: fail_data.charge_group_id,
            allowed_group_ids: fail_data.allowed_group_ids.clone(),
        }
    }
}

struct FailData<'a> {
    parent_item_key: ItemKey,
    charge_item_key: ItemKey,
    charge_group_id: ItemGrpId,
    allowed_group_ids: &'a Vec<ItemGrpId>,
}

impl VastFitData {
    // Fast validations
    pub(in crate::sol::svc::vast) fn validate_charge_group_fast(
        &mut self,
        kfs: &RSet<ItemKey>,
        uad: &Uad,
        fit: &UadFit,
    ) -> bool {
        iter_fails(kfs, uad, fit).next().is_none()
    }
    // Verbose validations
    pub(in crate::sol::svc::vast) fn validate_charge_group_verbose(
        &mut self,
        kfs: &RSet<ItemKey>,
        uad: &Uad,
        fit: &UadFit,
    ) -> Option<ValChargeGroupFail> {
        let mut charges = HashMap::new();
        for fail_data in iter_fails(kfs, uad, fit) {
            charges.insert(
                uad.items.id_by_key(fail_data.charge_item_key),
                ValChargeGroupChargeInfo::from_fail_data(uad, fail_data),
            );
        }
        match charges.is_empty() {
            true => None,
            false => Some(ValChargeGroupFail { charges }),
        }
    }
}

fn iter_fails<'a>(kfs: &RSet<ItemKey>, uad: &'a Uad, fit: &'a UadFit) -> impl Iterator<Item = FailData<'a>> {
    itertools::chain!(
        fit.mods_high.iter_keys().copied(),
        fit.mods_mid.iter_keys().copied(),
        fit.mods_low.iter_keys().copied(),
    )
    .filter_map(|module_item_key| {
        let uad_module = uad.items.get(module_item_key).get_module().unwrap();
        let charge_item_key = uad_module.get_charge_item_key()?;
        let charge_group_id = uad.items.get(charge_item_key).get_a_group_id()?;
        let allowed_group_ids = &uad_module.get_a_extras()?.charge_limit.as_ref()?.group_ids;
        if allowed_group_ids.contains(&charge_group_id) || kfs.contains(&charge_item_key) {
            return None;
        }
        Some(FailData {
            parent_item_key: module_item_key,
            charge_item_key,
            charge_group_id,
            allowed_group_ids,
        })
    })
}
