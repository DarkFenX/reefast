use crate::sol::{ItemKey, SolarSystem, api::ChargeMut};

impl SolarSystem {
    pub(in crate::sol::api) fn internal_remove_charge(&mut self, item_key: ItemKey) {
        let uad_item = self.uad.items.get(item_key);
        let uad_charge = uad_item.get_charge().unwrap();
        // Remove outgoing projections
        for &projectee_item_key in uad_charge.get_projs().iter_projectee_item_keys() {
            // Update services for charge
            let projectee_uad_item = self.uad.items.get(projectee_item_key);
            self.svc
                .remove_item_projection(&self.uad, item_key, projectee_item_key, projectee_uad_item);
            // Update user data for charge - do not touch projections container on charge itself,
            // because we're removing it anyway
            self.proj_tracker.unreg_projectee(&item_key, &projectee_item_key);
        }
        // Update services
        self.svc.remove_item(&self.uad, item_key, uad_item);
        // Update user data
        let module_item_key = uad_charge.get_cont_item_key();
        let uad_module = self.uad.items.get_mut(module_item_key).get_module_mut().unwrap();
        uad_module.set_charge_item_key(None);
        self.uad.items.remove(item_key);
    }
}

impl<'a> ChargeMut<'a> {
    pub fn remove(self) {
        self.sol.internal_remove_charge(self.key);
    }
}
