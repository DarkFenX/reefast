use crate::{
    defs::{EEffectId, SolItemId},
    err::basic::ItemFoundError,
    sol::{SolEffectMode, SolarSystem},
};

impl SolarSystem {
    pub fn set_item_effect_mode(
        &mut self,
        item_id: &SolItemId,
        effect_id: &EEffectId,
        mode: SolEffectMode,
    ) -> Result<(), SetItemEffectModeError> {
        self.uad
            .items
            .get_item_mut(item_id)?
            .get_effect_modes_mut()
            .set(*effect_id, mode);
        let item = self.uad.items.get_item(item_id).unwrap();
        self.svc.process_effects(&self.uad, item, item.get_state());
        Ok(())
    }
}

#[derive(Debug)]
pub enum SetItemEffectModeError {
    ItemNotFound(ItemFoundError),
}
impl std::error::Error for SetItemEffectModeError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::ItemNotFound(e) => Some(e),
        }
    }
}
impl std::fmt::Display for SetItemEffectModeError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::ItemNotFound(e) => e.fmt(f),
        }
    }
}
impl From<ItemFoundError> for SetItemEffectModeError {
    fn from(error: ItemFoundError) -> Self {
        Self::ItemNotFound(error)
    }
}
