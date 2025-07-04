use crate::{
    def::ItemKey,
    sol::{SolarSystem, reffs::REffs, rprojs::RProjs},
    svc::Svc,
    uad::Uad,
};

impl SolarSystem {
    pub(in crate::sol::api) fn util_add_fighter_with_projs(
        uad: &mut Uad,
        svc: &mut Svc,
        reffs: &mut REffs,
        rprojs: &mut RProjs,
        item_key: ItemKey,
    ) {
        // Process fighter itself
        let uad_item = uad.items.get(item_key);
        SolarSystem::util_add_item_with_projs(uad, svc, reffs, item_key, uad_item);
        // Process autocharges
        SolarSystem::add_fighter_autocharges(uad, svc, reffs, rprojs, item_key);
    }
    pub(in crate::sol::api) fn util_remove_fighter_with_projs(
        uad: &mut Uad,
        svc: &mut Svc,
        reffs: &mut REffs,
        rprojs: &mut RProjs,
        item_key: ItemKey,
    ) {
        // Process autocharges
        SolarSystem::remove_fighter_autocharges(uad, svc, reffs, rprojs, item_key, true);
        // Process fighter itself
        let uad_item = uad.items.get(item_key);
        SolarSystem::util_remove_item_with_projs(uad, svc, reffs, item_key, uad_item);
    }
}
