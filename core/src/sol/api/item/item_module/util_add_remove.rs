use crate::{
    def::ItemKey,
    sol::{SolarSystem, reffs::REffs},
    svc::Svc,
    uad::{Uad, UadItem},
};

impl SolarSystem {
    pub(in crate::sol::api) fn util_add_module_with_projs(
        uad: &Uad,
        svc: &mut Svc,
        reffs: &mut REffs,
        item_key: ItemKey,
        uad_item: &UadItem,
    ) {
        SolarSystem::util_add_item_with_projs(uad, svc, reffs, item_key, uad_item);
    }
    pub(in crate::sol::api) fn util_remove_module_with_projs(
        uad: &Uad,
        svc: &mut Svc,
        reffs: &mut REffs,
        item_key: ItemKey,
        uad_item: &UadItem,
    ) {
        SolarSystem::util_remove_item_with_projs(uad, svc, reffs, item_key, uad_item);
    }
}
