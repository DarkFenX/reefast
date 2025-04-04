use crate::{
    err::basic::{ItemFoundError, ItemKindMatchError},
    sol::{ItemId, SolarSystem, info::AutochargeInfo},
};

impl SolarSystem {
    pub fn get_autocharge(&self, item_id: &ItemId) -> Result<AutochargeInfo, GetAutochargeError> {
        let autocharge = self.uad.items.get_item(item_id)?.get_autocharge()?;
        Ok(AutochargeInfo::from(autocharge))
    }
}

#[derive(Debug)]
pub enum GetAutochargeError {
    ItemNotFound(ItemFoundError),
    ItemIsNotAutocharge(ItemKindMatchError),
}
impl std::error::Error for GetAutochargeError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::ItemNotFound(e) => Some(e),
            Self::ItemIsNotAutocharge(e) => Some(e),
        }
    }
}
impl std::fmt::Display for GetAutochargeError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::ItemNotFound(e) => e.fmt(f),
            Self::ItemIsNotAutocharge(e) => e.fmt(f),
        }
    }
}
impl From<ItemFoundError> for GetAutochargeError {
    fn from(error: ItemFoundError) -> Self {
        Self::ItemNotFound(error)
    }
}
impl From<ItemKindMatchError> for GetAutochargeError {
    fn from(error: ItemKindMatchError) -> Self {
        Self::ItemIsNotAutocharge(error)
    }
}
