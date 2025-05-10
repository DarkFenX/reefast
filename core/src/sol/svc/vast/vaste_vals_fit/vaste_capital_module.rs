use std::collections::HashMap;

use crate::{
    ac, ad,
    sol::{
        AttrVal, ItemId, ItemKey,
        svc::vast::VastFitData,
        uad::{Uad, fit::UadFit, item::UadShip},
    },
    util::RSet,
};

pub struct ValCapitalModFail {
    /// Modules up to and including this volume are not considered capital.
    pub max_subcap_volume: AttrVal,
    /// List of modules breaking validation, and their volumes.
    pub module_volumes: HashMap<ItemId, AttrVal>,
}

impl VastFitData {
    // Fast validations
    pub(in crate::sol::svc::vast) fn validate_capital_module_fast(
        &self,
        kfs: &RSet<ItemKey>,
        uad: &Uad,
        fit: &UadFit,
        ship: Option<&UadShip>,
    ) -> bool {
        if !is_ship_subcap(ship) {
            return true;
        }
        iter_capital_modules(kfs, uad, fit).next().is_none()
    }
    // Verbose validations
    pub(in crate::sol::svc::vast) fn validate_capital_module_verbose(
        &self,
        kfs: &RSet<ItemKey>,
        uad: &Uad,
        fit: &UadFit,
        ship: Option<&UadShip>,
    ) -> Option<ValCapitalModFail> {
        if !is_ship_subcap(ship) {
            return None;
        }
        let module_volumes: HashMap<_, _> = iter_capital_modules(kfs, uad, fit)
            .map(|module_key| {
                let uad_module = uad.items.get(module_key).get_module().unwrap();
                (
                    uad_module.get_item_id(),
                    uad_module.get_a_extras().unwrap().volume.unwrap(),
                )
            })
            .collect();
        match module_volumes.is_empty() {
            true => None,
            false => Some(ValCapitalModFail {
                max_subcap_volume: ac::extras::MAX_SUBCAP_MODULE_VOLUME,
                module_volumes,
            }),
        }
    }
}

fn is_ship_subcap(ship: Option<&UadShip>) -> bool {
    let ship = match ship {
        Some(ship) => ship,
        None => return false,
    };
    let extras = match ship.get_a_extras() {
        Some(extras) => extras,
        None => return false,
    };
    matches!(extras.ship_kind, Some(ad::AShipKind::Ship))
}

fn iter_capital_modules(kfs: &RSet<ItemKey>, uad: &Uad, fit: &UadFit) -> impl Iterator<Item = ItemKey> {
    itertools::chain!(
        fit.mods_high.iter_keys().copied(),
        fit.mods_mid.iter_keys().copied(),
        fit.mods_low.iter_keys().copied(),
    )
    .filter(|item_key| {
        let uad_item = uad.items.get(*item_key);
        match uad_item.get_a_extras() {
            Some(a_extras) => {
                matches!(a_extras.item_ship_kind, Some(ad::AShipKind::CapitalShip)) && !kfs.contains(item_key)
            }
            None => false,
        }
    })
}
