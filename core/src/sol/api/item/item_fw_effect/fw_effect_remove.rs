use crate::{
    def::ItemKey,
    sol::{SolarSystem, api::FwEffectMut},
};

impl SolarSystem {
    pub(in crate::sol::api) fn internal_remove_fw_effect(&mut self, item_key: ItemKey) {
        let uad_item = self.uad.items.get(item_key);
        let uad_fw_effect = uad_item.get_fw_effect().unwrap();
        SolarSystem::util_remove_item_without_projs(&self.uad, &mut self.svc, &mut self.reffs, item_key, uad_item);
        let uad_fit = self.uad.fits.get_mut(uad_fw_effect.get_fit_key());
        uad_fit.fw_effects.remove(&item_key);
        self.uad.items.remove(item_key);
    }
}

impl<'a> FwEffectMut<'a> {
    pub fn remove(self) {
        self.sol.internal_remove_fw_effect(self.key);
    }
}
