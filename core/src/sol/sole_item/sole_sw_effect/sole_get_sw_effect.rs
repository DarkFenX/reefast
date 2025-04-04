use crate::{
    err::basic::{ItemFoundError, ItemKindMatchError},
    sol::{ItemId, SolarSystem, info::SwEffectInfo},
};

impl SolarSystem {
    pub fn get_sw_effect(&self, item_id: &ItemId) -> Result<SwEffectInfo, GetSwEffectError> {
        let sw_effect = self.uad.items.get_item(item_id)?.get_sw_effect()?;
        Ok(SwEffectInfo::from(sw_effect))
    }
}

#[derive(Debug)]
pub enum GetSwEffectError {
    ItemNotFound(ItemFoundError),
    ItemIsNotSwEffect(ItemKindMatchError),
}
impl std::error::Error for GetSwEffectError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::ItemNotFound(e) => Some(e),
            Self::ItemIsNotSwEffect(e) => Some(e),
        }
    }
}
impl std::fmt::Display for GetSwEffectError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::ItemNotFound(e) => e.fmt(f),
            Self::ItemIsNotSwEffect(e) => e.fmt(f),
        }
    }
}
impl From<ItemFoundError> for GetSwEffectError {
    fn from(error: ItemFoundError) -> Self {
        Self::ItemNotFound(error)
    }
}
impl From<ItemKindMatchError> for GetSwEffectError {
    fn from(error: ItemKindMatchError) -> Self {
        Self::ItemIsNotSwEffect(error)
    }
}
