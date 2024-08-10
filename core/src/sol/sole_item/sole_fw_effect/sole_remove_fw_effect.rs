use crate::{
    defs::SolItemId,
    sol::{
        err::basic::{ItemFoundError, ItemKindMatchError},
        SolView, SolarSystem,
    },
};

impl SolarSystem {
    pub fn remove_fw_effect(&mut self, item_id: &SolItemId) -> Result<(), RemoveFwEffectError> {
        let item = self.items.get_item(item_id)?;
        let fw_effect = item.get_fw_effect()?;
        self.svcs
            .remove_item(&SolView::new(&self.src, &self.fleets, &self.fits, &self.items), item);
        let fit = self.fits.get_fit_mut(&fw_effect.get_fit_id()).unwrap();
        fit.fw_effects.remove(item_id);
        self.items.remove_item(item_id);
        Ok(())
    }
}

#[derive(Debug)]
pub enum RemoveFwEffectError {
    ItemNotFound(ItemFoundError),
    ItemIsNotFwEffect(ItemKindMatchError),
}
impl From<ItemFoundError> for RemoveFwEffectError {
    fn from(error: ItemFoundError) -> Self {
        Self::ItemNotFound(error)
    }
}
impl From<ItemKindMatchError> for RemoveFwEffectError {
    fn from(error: ItemKindMatchError) -> Self {
        Self::ItemIsNotFwEffect(error)
    }
}
impl std::error::Error for RemoveFwEffectError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::ItemNotFound(e) => Some(e),
            Self::ItemIsNotFwEffect(e) => Some(e),
        }
    }
}
impl std::fmt::Display for RemoveFwEffectError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::ItemNotFound(e) => e.fmt(f),
            Self::ItemIsNotFwEffect(e) => e.fmt(f),
        }
    }
}
