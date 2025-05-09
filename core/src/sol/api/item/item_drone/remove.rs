use crate::sol::{ItemKey, SolarSystem, api::DroneMut};

impl SolarSystem {
    pub(in crate::sol::api) fn internal_remove_drone(&mut self, item_key: ItemKey) {
        // Just check if everything is correct
        let uad_item = self.uad.items.get(item_key);
        let uad_drone = uad_item.get_drone().unwrap();
        let fit_key = uad_drone.get_fit_key();
        // Remove outgoing projections
        for &projectee_item_key in uad_drone.get_projs().iter_projectee_item_keys() {
            // Update services
            let projectee_uad_item = self.uad.items.get(projectee_item_key);
            self.svc
                .remove_item_projection(&self.uad, item_key, projectee_item_key, projectee_uad_item);
            // Update user data - do not update info on drone, because drone will be discarded
            // anyway
            self.proj_tracker.unreg_projectee(&item_key, &projectee_item_key);
        }
        // Remove incoming projections
        self.internal_remove_incoming_projections(item_key);
        // Remove drone from services
        self.internal_remove_item_key_from_svc(item_key);
        // Remove drone from user data
        let uad_fit = self.uad.fits.get_mut(fit_key);
        uad_fit.drones.remove(&item_key);
        self.uad.items.remove(item_key);
    }
}

impl<'a> DroneMut<'a> {
    pub fn remove(self) {
        self.sol.internal_remove_drone(self.key);
    }
}
