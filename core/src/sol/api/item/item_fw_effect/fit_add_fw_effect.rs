use crate::{
    ad,
    def::ItemTypeId,
    sol::{
        SolarSystem,
        api::{FitMut, FwEffectMut},
    },
    ud::{UEffectUpdates, UFitKey, UFwEffect, UItem, UItemKey},
};

impl SolarSystem {
    pub(in crate::sol::api) fn internal_add_fw_effect(
        &mut self,
        fit_key: UFitKey,
        a_item_id: ad::AItemId,
        reuse_eupdates: &mut UEffectUpdates,
    ) -> UItemKey {
        let u_fit = self.u_data.fits.get_mut(fit_key);
        let item_id = self.u_data.items.alloc_id();
        let u_fw_effect = UFwEffect::new(item_id, a_item_id, fit_key, true, &self.u_data.src, reuse_eupdates);
        let u_item = UItem::FwEffect(u_fw_effect);
        let item_key = self.u_data.items.add(u_item);
        u_fit.fw_effects.insert(item_key);
        let u_item = self.u_data.items.get(item_key);
        SolarSystem::util_add_item_without_projs(&self.u_data, &mut self.svc, item_key, u_item, reuse_eupdates);
        item_key
    }
}

impl<'a> FitMut<'a> {
    pub fn add_fw_effect(&mut self, type_id: ItemTypeId) -> FwEffectMut<'_> {
        let mut reuse_eupdates = UEffectUpdates::new();
        let item_key = self.sol.internal_add_fw_effect(self.key, type_id, &mut reuse_eupdates);
        FwEffectMut::new(self.sol, item_key)
    }
}
