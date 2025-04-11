use crate::{
    err::basic::{ItemFoundError, ItemKindMatchError},
    sol::{ItemId, ItemKey, SolarSystem, info::DroneInfo},
};

impl SolarSystem {
    pub fn get_drone_info(&self, item_id: &ItemId) -> Result<DroneInfo, GetDroneInfoError> {
        let item_key = self.uad.items.key_by_id_err(item_id)?;
        Ok(self.get_drone_info_internal(item_key)?)
    }
    pub(in crate::sol) fn get_drone_info_internal(&self, item_key: ItemKey) -> Result<DroneInfo, ItemKindMatchError> {
        let drone = self.uad.items.get(item_key).get_drone()?;
        Ok(DroneInfo::from_drone(&self.uad, drone))
    }
}

#[derive(thiserror::Error, Debug)]
pub enum GetDroneInfoError {
    #[error("{0}")]
    ItemNotFound(#[from] ItemFoundError),
    #[error("{0}")]
    ItemIsNotDrone(#[from] ItemKindMatchError),
}
