use crate::{
    err::basic::{ItemFoundError, ItemKindMatchError, ItemMutatedError},
    sol::{ItemId, SolarSystem},
};

impl SolarSystem {
    pub fn remove_drone_mutation(&mut self, item_id: &ItemId) -> Result<(), RemoveDroneMutationError> {
        let item = self.uad.items.get_item(item_id)?;
        let drone = item.get_drone()?;
        if !drone.has_mutation_data() {
            return Err(ItemMutatedError { item_id: *item_id }.into());
        }
        self.svc.unload_item(&self.uad, item);
        self.uad
            .items
            .get_item_mut(item_id)
            .unwrap()
            .get_drone_mut()
            .unwrap()
            .unmutate(&self.uad.src)
            .unwrap();
        let item = self.uad.items.get_item(item_id).unwrap();
        self.svc.load_item(&self.uad, item);
        Ok(())
    }
}

#[derive(Debug)]
pub enum RemoveDroneMutationError {
    ItemNotFound(ItemFoundError),
    ItemIsNotDrone(ItemKindMatchError),
    MutationNotSet(ItemMutatedError),
}
impl std::error::Error for RemoveDroneMutationError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::ItemNotFound(e) => Some(e),
            Self::ItemIsNotDrone(e) => Some(e),
            Self::MutationNotSet(e) => Some(e),
        }
    }
}
impl std::fmt::Display for RemoveDroneMutationError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::ItemNotFound(e) => e.fmt(f),
            Self::ItemIsNotDrone(e) => e.fmt(f),
            Self::MutationNotSet(e) => e.fmt(f),
        }
    }
}
impl From<ItemFoundError> for RemoveDroneMutationError {
    fn from(error: ItemFoundError) -> Self {
        Self::ItemNotFound(error)
    }
}
impl From<ItemKindMatchError> for RemoveDroneMutationError {
    fn from(error: ItemKindMatchError) -> Self {
        Self::ItemIsNotDrone(error)
    }
}
impl From<ItemMutatedError> for RemoveDroneMutationError {
    fn from(error: ItemMutatedError) -> Self {
        Self::MutationNotSet(error)
    }
}
