use crate::{
    ac, ad,
    sol::{
        ItemId, ItemKey,
        svc::{calc::Calc, vast::VastFitData},
        uad::{Uad, fit::UadFit},
    },
    util::RSet,
};

use super::shared::is_flag_set;

pub struct ValActivationBlockedFail {
    /// Item IDs of modules which are active, but their activation is blocked by something.
    pub module_ids: Vec<ItemId>,
}

impl VastFitData {
    // Fast validations
    pub(in crate::sol::svc::vast) fn validate_activation_blocked_fast(
        &self,
        kfs: &RSet<ItemKey>,
        uad: &Uad,
        calc: &mut Calc,
        fit: &UadFit,
    ) -> bool {
        iter_active_modules(uad, fit).all(|item_key| {
            !is_flag_set(uad, calc, item_key, &ac::attrs::ACTIVATION_BLOCKED) || kfs.contains(&item_key)
        })
    }
    // Verbose validations
    pub(in crate::sol::svc::vast) fn validate_activation_blocked_verbose(
        &self,
        kfs: &RSet<ItemKey>,
        uad: &Uad,
        calc: &mut Calc,
        fit: &UadFit,
    ) -> Option<ValActivationBlockedFail> {
        let module_ids: Vec<_> = iter_active_modules(uad, fit)
            .filter(|item_key| {
                is_flag_set(uad, calc, *item_key, &ac::attrs::ACTIVATION_BLOCKED) && !kfs.contains(item_key)
            })
            .map(|item_key| uad.items.id_by_key(item_key))
            .collect();
        match module_ids.is_empty() {
            true => None,
            false => Some(ValActivationBlockedFail { module_ids }),
        }
    }
}

fn iter_active_modules(uad: &Uad, fit: &UadFit) -> impl Iterator<Item = ItemKey> {
    itertools::chain!(
        fit.mods_high.iter_keys().copied(),
        fit.mods_mid.iter_keys().copied(),
        fit.mods_low.iter_keys().copied(),
    )
    .filter(|item_key| {
        let uad_module = uad.items.get(*item_key).get_module().unwrap();
        uad_module.get_a_state() >= ad::AState::Active && uad_module.is_loaded()
    })
}
