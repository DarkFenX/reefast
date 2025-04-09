use crate::{
    err::basic::{ItemFoundError, ItemKindMatchError},
    sol::{ItemId, ItemKey, SolarSystem},
};

impl SolarSystem {
    pub fn set_autocharge_state(&mut self, item_id: &ItemId, state: bool) -> Result<(), SetAutochargeStateError> {
        let item_key = self.uad.items.key_by_id_err(item_id)?;
        Ok(self.set_autocharge_state_internal(item_key, state)?)
    }
    pub(in crate::sol) fn set_autocharge_state_internal(
        &mut self,
        item_key: ItemKey,
        state: bool,
    ) -> Result<(), ItemKindMatchError> {
        let autocharge = self.uad.items.get_mut(item_key).get_autocharge_mut()?;
        let old_a_state = autocharge.get_a_state();
        autocharge.set_force_disable(!state);
        let new_a_state = autocharge.get_a_state();
        self.change_item_key_state_in_svc(item_key, old_a_state, new_a_state);
        Ok(())
    }
}

#[derive(thiserror::Error, Debug)]
pub enum SetAutochargeStateError {
    #[error("{0}")]
    ItemNotFound(#[from] ItemFoundError),
    #[error("{0}")]
    ItemIsNotAutocharge(#[from] ItemKindMatchError),
}
