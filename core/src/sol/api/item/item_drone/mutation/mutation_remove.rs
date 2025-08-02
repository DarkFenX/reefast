use crate::{
    sol::SolarSystem,
    ud::{UEffectUpdates, UItemKey, err::ItemMutatedError},
};

impl SolarSystem {
    pub(in crate::sol::api) fn internal_remove_drone_mutation(
        &mut self,
        item_key: UItemKey,
        reuse_eupdates: &mut UEffectUpdates,
    ) -> Result<(), ItemMutatedError> {
        SolarSystem::util_remove_drone(&mut self.u_data, &mut self.svc, item_key, reuse_eupdates);
        let u_drone = self.u_data.items.get_mut(item_key).get_drone_mut().unwrap();
        if let Err(error) = u_drone.unmutate(&self.u_data.src) {
            SolarSystem::util_add_drone(&mut self.u_data, &mut self.svc, item_key, reuse_eupdates);
            return Err(error);
        }
        SolarSystem::util_update_item_radius_in_projs(&mut self.u_data, &self.rev_projs, &mut self.svc, item_key);
        SolarSystem::util_add_drone(&mut self.u_data, &mut self.svc, item_key, reuse_eupdates);
        Ok(())
    }
}
