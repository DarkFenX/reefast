use crate::{
    err::basic::{ItemFoundError, ItemKindMatchError, ItemNotMutatedError},
    sol::{ItemId, SolarSystem, uad::item::ItemAddMutation},
};

impl SolarSystem {
    pub fn add_drone_mutation(
        &mut self,
        item_id: &ItemId,
        mutation: ItemAddMutation,
    ) -> Result<(), AddDroneMutationError> {
        let item = self.uad.items.get_item(item_id)?;
        self.svc.unload_item(&self.uad, item);
        let drone = match self.uad.items.get_item_mut(item_id).unwrap().get_drone_mut() {
            Ok(drone) => drone,
            Err(error) => {
                let item = self.uad.items.get_item(item_id).unwrap();
                self.svc.load_item(&self.uad, item);
                return Err(error.into());
            }
        };
        if let Err(error) = drone.mutate(&self.uad.src, mutation) {
            let item = self.uad.items.get_item(item_id).unwrap();
            self.svc.load_item(&self.uad, item);
            return Err(error.into());
        }
        let item = self.uad.items.get_item(item_id).unwrap();
        self.svc.load_item(&self.uad, item);
        Ok(())
    }
}

#[derive(Debug)]
pub enum AddDroneMutationError {
    ItemNotFound(ItemFoundError),
    ItemIsNotDrone(ItemKindMatchError),
    MutationAlreadySet(ItemNotMutatedError),
}
impl std::error::Error for AddDroneMutationError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::ItemNotFound(e) => Some(e),
            Self::ItemIsNotDrone(e) => Some(e),
            Self::MutationAlreadySet(e) => Some(e),
        }
    }
}
impl std::fmt::Display for AddDroneMutationError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::ItemNotFound(e) => e.fmt(f),
            Self::ItemIsNotDrone(e) => e.fmt(f),
            Self::MutationAlreadySet(e) => e.fmt(f),
        }
    }
}
impl From<ItemFoundError> for AddDroneMutationError {
    fn from(error: ItemFoundError) -> Self {
        Self::ItemNotFound(error)
    }
}
impl From<ItemKindMatchError> for AddDroneMutationError {
    fn from(error: ItemKindMatchError) -> Self {
        Self::ItemIsNotDrone(error)
    }
}
impl From<ItemNotMutatedError> for AddDroneMutationError {
    fn from(error: ItemNotMutatedError) -> Self {
        Self::MutationAlreadySet(error)
    }
}
