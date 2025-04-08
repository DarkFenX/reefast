use crate::{
    err::basic::{ItemFoundError, ItemKindMatchError},
    sol::{ItemId, ItemKey, SolarSystem, info::BoosterInfo},
};

impl SolarSystem {
    pub fn get_booster(&self, item_id: &ItemId) -> Result<BoosterInfo, GetBoosterError> {
        let item_key = self.uad.items.key_by_id_err(item_id)?;
        Ok(self.get_booster_internal(item_key)?)
    }
    pub(in crate::sol) fn get_booster_internal(&self, item_key: ItemKey) -> Result<BoosterInfo, ItemKindMatchError> {
        let booster = self.uad.items.get(item_key).get_booster()?;
        Ok(BoosterInfo::from_booster(&self.uad, booster))
    }
}

#[derive(Debug)]
pub enum GetBoosterError {
    ItemNotFound(ItemFoundError),
    ItemIsNotBooster(ItemKindMatchError),
}
impl std::error::Error for GetBoosterError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::ItemNotFound(e) => Some(e),
            Self::ItemIsNotBooster(e) => Some(e),
        }
    }
}
impl std::fmt::Display for GetBoosterError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::ItemNotFound(e) => e.fmt(f),
            Self::ItemIsNotBooster(e) => e.fmt(f),
        }
    }
}
impl From<ItemFoundError> for GetBoosterError {
    fn from(error: ItemFoundError) -> Self {
        Self::ItemNotFound(error)
    }
}
impl From<ItemKindMatchError> for GetBoosterError {
    fn from(error: ItemKindMatchError) -> Self {
        Self::ItemIsNotBooster(error)
    }
}
