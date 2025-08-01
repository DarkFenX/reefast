use crate::{
    ad,
    sol::SolarSystem,
    ud::{UEffectUpdates, UItemKey, err::ItemMutatedError},
};

impl SolarSystem {
    pub(in crate::sol::api) fn internal_set_module_a_mutator_id(
        &mut self,
        item_key: UItemKey,
        a_mutator_id: ad::AItemId,
        reuse_eupdates: &mut UEffectUpdates,
    ) -> Result<(), ItemMutatedError> {
        let u_item = self.u_data.items.get(item_key);
        let mutation_data = match u_item.get_mutation_data() {
            Some(mutation_data) => mutation_data,
            None => {
                return Err(ItemMutatedError {
                    item_id: self.u_data.items.id_by_key(item_key),
                });
            }
        };
        if mutation_data.get_mutator_id() == a_mutator_id {
            return Ok(());
        }
        SolarSystem::util_remove_module_with_projs(&self.u_data, &mut self.svc, item_key, u_item, reuse_eupdates);
        self.u_data
            .items
            .get_mut(item_key)
            .get_module_mut()
            .unwrap()
            .set_mutator_id(a_mutator_id, reuse_eupdates, &self.u_data.src)
            .unwrap();
        SolarSystem::util_add_module_with_projs(&self.u_data, &mut self.svc, item_key, reuse_eupdates);
        Ok(())
    }
}
