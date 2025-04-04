use crate::{
    err::basic::{ItemFoundError, ItemKindMatchError},
    sol::{EffectId, EffectMode, ItemId, SolarSystem},
};

impl SolarSystem {
    pub fn set_booster_side_effect_state(
        &mut self,
        item_id: &ItemId,
        effect_id: &EffectId,
        state: bool,
    ) -> Result<(), SetBoosterSideEffectStateError> {
        let booster = self.uad.items.get_item_mut(item_id)?.get_booster_mut()?;
        let effect_state = match state {
            true => EffectMode::StateCompliance,
            false => EffectMode::FullCompliance,
        };
        booster.get_effect_modes_mut().set(effect_id.into(), effect_state);
        let item = self.uad.items.get_item(item_id).unwrap();
        self.svc.process_effects(&self.uad, item, item.get_a_state());
        Ok(())
    }
}

#[derive(Debug)]
pub enum SetBoosterSideEffectStateError {
    ItemNotFound(ItemFoundError),
    ItemIsNotBooster(ItemKindMatchError),
}
impl From<ItemFoundError> for SetBoosterSideEffectStateError {
    fn from(error: ItemFoundError) -> Self {
        Self::ItemNotFound(error)
    }
}
impl From<ItemKindMatchError> for SetBoosterSideEffectStateError {
    fn from(error: ItemKindMatchError) -> Self {
        Self::ItemIsNotBooster(error)
    }
}
impl std::error::Error for SetBoosterSideEffectStateError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::ItemNotFound(e) => Some(e),
            Self::ItemIsNotBooster(e) => Some(e),
        }
    }
}
impl std::fmt::Display for SetBoosterSideEffectStateError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::ItemNotFound(e) => e.fmt(f),
            Self::ItemIsNotBooster(e) => e.fmt(f),
        }
    }
}
