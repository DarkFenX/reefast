use crate::{
    err::basic::ProjFoundError,
    sol::{ItemKey, SolarSystem},
};

impl SolarSystem {
    pub(in crate::sol::api) fn internal_remove_module_proj(
        &mut self,
        item_key: ItemKey,
        projectee_item_key: ItemKey,
    ) -> Result<(), ProjFoundError> {
        // Check if projection is defined
        let uad_module = self.uad.items.get(item_key).get_module().unwrap();
        if !uad_module.get_projs().contains(&projectee_item_key) {
            return Err(ProjFoundError {
                projector_item_id: uad_module.get_item_id(),
                projectee_item_id: self.uad.items.id_by_key(projectee_item_key),
            });
        };
        let charge_key = uad_module.get_charge_item_key();
        if let Some(charge_key) = charge_key {
            // Update services for charge
            self.internal_remove_item_key_projection_from_svc(charge_key, projectee_item_key);
            // Update user data for charge
            self.proj_tracker.unreg_projectee(&charge_key, &projectee_item_key);
            let uad_charge = self.uad.items.get_mut(charge_key).get_charge_mut().unwrap();
            uad_charge.get_projs_mut().remove(&projectee_item_key);
        }
        // Update services for module
        self.internal_remove_item_key_projection_from_svc(item_key, projectee_item_key);
        // Update user data for module
        self.proj_tracker.unreg_projectee(&item_key, &projectee_item_key);
        let uad_module = self.uad.items.get_mut(item_key).get_module_mut().unwrap();
        uad_module.get_projs_mut().remove(&projectee_item_key);
        Ok(())
    }
}
