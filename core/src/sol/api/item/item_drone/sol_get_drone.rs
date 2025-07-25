use crate::{
    def::ItemId,
    err::basic::{ItemFoundError, ItemKindMatchError},
    sol::{
        SolarSystem,
        api::{Drone, DroneMut},
    },
};

impl SolarSystem {
    pub fn get_drone(&self, item_id: &ItemId) -> Result<Drone<'_>, GetDroneError> {
        let item_key = self.u_data.items.key_by_id_err(item_id)?;
        self.u_data.items.get(item_key).get_drone()?;
        Ok(Drone::new(self, item_key))
    }
    pub fn get_drone_mut(&mut self, item_id: &ItemId) -> Result<DroneMut<'_>, GetDroneError> {
        let item_key = self.u_data.items.key_by_id_err(item_id)?;
        self.u_data.items.get(item_key).get_drone()?;
        Ok(DroneMut::new(self, item_key))
    }
}

#[derive(thiserror::Error, Debug)]
pub enum GetDroneError {
    #[error("{0}")]
    ItemNotFound(#[from] ItemFoundError),
    #[error("{0}")]
    ItemIsNotDrone(#[from] ItemKindMatchError),
}
