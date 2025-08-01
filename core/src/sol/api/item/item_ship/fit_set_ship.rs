use crate::{
    ad,
    def::{ItemTypeId, OF},
    sol::{
        SolarSystem,
        api::{FitMut, ShipMut},
    },
    ud::{UEffectUpdates, UFitKey, UItem, UItemKey, UShip},
};

impl SolarSystem {
    pub(in crate::sol::api) fn internal_set_fit_ship(
        &mut self,
        fit_key: UFitKey,
        a_item_id: ad::AItemId,
        reuse_eupdates: &mut UEffectUpdates,
    ) -> UItemKey {
        let u_fit = self.u_data.fits.get(fit_key);
        // Remove old ship, if it was set
        if let Some(old_item_key) = u_fit.ship {
            self.internal_remove_ship(old_item_key, reuse_eupdates);
        }
        // Add new ship
        let item_id = self.u_data.items.alloc_id();
        let u_ship = UShip::new(item_id, a_item_id, fit_key, true, &self.u_data.src, reuse_eupdates);
        let ship_kind = u_ship.get_kind();
        let ship_radius = u_ship.get_axt().map(|v| v.radius).unwrap_or(OF(0.0));
        let u_item = UItem::Ship(u_ship);
        let item_key = self.u_data.items.add(u_item);
        let u_fit = self.u_data.fits.get_mut(fit_key);
        u_fit.ship = Some(item_key);
        u_fit.kind = ship_kind;
        SolarSystem::util_add_ship(&self.u_data, &mut self.svc, item_key, reuse_eupdates);
        // Update projections outgoing from on-ship items
        SolarSystem::util_update_ship_radius_for_outgoing_projs(&mut self.u_data, &mut self.svc, fit_key, ship_radius);
        item_key
    }
}

impl<'a> FitMut<'a> {
    pub fn set_ship(&mut self, type_id: ItemTypeId) -> ShipMut<'_> {
        let mut reuse_eupdates = UEffectUpdates::new();
        let item_key = self.sol.internal_set_fit_ship(self.key, type_id, &mut reuse_eupdates);
        ShipMut::new(self.sol, item_key)
    }
}
