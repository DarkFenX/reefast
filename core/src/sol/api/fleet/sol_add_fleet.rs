use crate::{
    def::FleetKey,
    sol::{SolarSystem, api::FleetMut},
    uad::UadFleet,
};

impl SolarSystem {
    pub fn add_fleet(&mut self) -> FleetMut<'_> {
        let fleet_key = self.add_fleet_internal();
        FleetMut::new(self, fleet_key)
    }
    pub(in crate::sol::api) fn add_fleet_internal(&mut self) -> FleetKey {
        let fleet_id = self.uad.fleets.alloc_id();
        let uad_fleet = UadFleet::new(fleet_id);
        self.uad.fleets.add(uad_fleet)
    }
}
