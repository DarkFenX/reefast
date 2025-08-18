use crate::{
    ad::AItemId,
    def::ItemTypeId,
    sol::{SolarSystem, api::RigMut},
    ud::{UEffectUpdates, UItemKey},
};

impl SolarSystem {
    pub(in crate::sol::api) fn internal_set_rig_type_id(
        &mut self,
        rig_key: UItemKey,
        type_id: AItemId,
        reuse_eupdates: &mut UEffectUpdates,
    ) {
        let u_item = self.u_data.items.get(rig_key);
        if u_item.get_type_id() == type_id {
            return;
        }
        SolarSystem::util_remove_rig(&mut self.u_data, &mut self.svc, rig_key, reuse_eupdates);
        let u_rig = self.u_data.items.get_mut(rig_key).get_rig_mut().unwrap();
        u_rig.set_type_id(type_id, &self.u_data.src);
        SolarSystem::util_add_rig(&mut self.u_data, &mut self.svc, rig_key, reuse_eupdates);
    }
}

impl<'a> RigMut<'a> {
    /// Set type ID, replacing currently used EVE item by another, preserving all the user data.
    pub fn set_type_id(&mut self, type_id: ItemTypeId) {
        let mut reuse_eupdates = UEffectUpdates::new();
        self.sol
            .internal_set_rig_type_id(self.key, type_id, &mut reuse_eupdates)
    }
}
