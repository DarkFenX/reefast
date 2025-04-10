use crate::{
    err::basic::{ItemFoundError, ItemKindMatchError},
    sol::{ItemId, ItemKey, SolarSystem, uad::item::ModuleState},
};

impl SolarSystem {
    pub fn set_module_state(&mut self, item_id: &ItemId, state: ModuleState) -> Result<(), SetModuleStateError> {
        let item_key = self.uad.items.key_by_id_err(item_id)?;
        Ok(self.set_module_state_internal(item_key, state)?)
    }
    pub(in crate::sol) fn set_module_state_internal(
        &mut self,
        item_key: ItemKey,
        state: ModuleState,
    ) -> Result<(), ItemKindMatchError> {
        // Update user data for module
        let module = self.uad.items.get_mut(item_key).get_module_mut()?;
        let charge_key = module.get_charge_item_key();
        let old_a_state = module.get_a_state();
        module.set_module_state(state);
        let new_a_state = module.get_a_state();
        // Update services for module
        self.change_item_key_state_in_svc(item_key, old_a_state, new_a_state);
        if let Some(charge_key) = charge_key {
            // Update user data for charge
            let charge = self.uad.items.get_mut(charge_key).get_charge_mut().unwrap();
            let old_a_state = charge.get_a_state();
            charge.set_a_state(state.into());
            let new_a_state = charge.get_a_state();
            // Update services for charge
            self.change_item_key_state_in_svc(charge_key, old_a_state, new_a_state);
        }
        Ok(())
    }
}

#[derive(thiserror::Error, Debug)]
pub enum SetModuleStateError {
    #[error("{0}")]
    ItemNotFound(#[from] ItemFoundError),
    #[error("{0}")]
    ItemIsNotModule(#[from] ItemKindMatchError),
}
