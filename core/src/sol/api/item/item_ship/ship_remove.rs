use crate::{
    def::{ItemKey, OF},
    sol::{SolarSystem, api::ShipMut},
    uad::ShipKind,
};

impl SolarSystem {
    pub(in crate::sol::api) fn internal_remove_ship(&mut self, item_key: ItemKey) {
        let uad_ship = self.uad.items.get(item_key).get_ship().unwrap();
        let fit_key = uad_ship.get_fit_key();
        // Remove incoming projections
        self.internal_remove_incoming_projections(item_key);
        // Remove ship from services
        let uad_item = self.uad.items.get(item_key);
        SolarSystem::util_remove_ship(&self.uad, &mut self.svc, &mut self.reffs, item_key, uad_item);
        // Remove ship from user data
        let uad_fit = self.uad.fits.get_mut(fit_key);
        uad_fit.ship = None;
        uad_fit.kind = ShipKind::Unknown;
        self.uad.items.remove(item_key);
        // Update projections outgoing from on-ship items
        SolarSystem::util_update_ship_radius_for_outgoing_projs(
            &mut self.uad,
            &mut self.svc,
            &self.reffs,
            fit_key,
            OF(0.0),
        );
    }
}

impl<'a> ShipMut<'a> {
    pub fn remove(self) {
        self.sol.internal_remove_ship(self.key);
    }
}
