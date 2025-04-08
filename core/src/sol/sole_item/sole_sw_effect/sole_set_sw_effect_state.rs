use crate::{
    err::basic::{ItemFoundError, ItemKindMatchError},
    sol::{ItemId, ItemKey, SolarSystem},
};

impl SolarSystem {
    pub fn set_sw_effect_state(&mut self, item_id: &ItemId, state: bool) -> Result<(), SetSwEffectStateError> {
        let item_key = self.uad.items.key_by_id_err(item_id)?;
        Ok(self.set_sw_effect_state_internal(item_key, state)?)
    }
    pub(in crate::sol) fn set_sw_effect_state_internal(
        &mut self,
        item_key: ItemKey,
        state: bool,
    ) -> Result<(), ItemKindMatchError> {
        let sw_effect = self.uad.items.get_mut(item_key).get_sw_effect_mut()?;
        let old_a_state = sw_effect.get_a_state();
        sw_effect.set_sw_effect_state(state);
        let new_a_state = sw_effect.get_a_state();
        self.change_item_key_state_in_svc(item_key, old_a_state, new_a_state);
        Ok(())
    }
}

#[derive(Debug)]
pub enum SetSwEffectStateError {
    ItemNotFound(ItemFoundError),
    ItemIsNotSwEffect(ItemKindMatchError),
}
impl std::error::Error for SetSwEffectStateError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::ItemNotFound(e) => Some(e),
            Self::ItemIsNotSwEffect(e) => Some(e),
        }
    }
}
impl std::fmt::Display for SetSwEffectStateError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::ItemNotFound(e) => e.fmt(f),
            Self::ItemIsNotSwEffect(e) => e.fmt(f),
        }
    }
}
impl From<ItemFoundError> for SetSwEffectStateError {
    fn from(error: ItemFoundError) -> Self {
        Self::ItemNotFound(error)
    }
}
impl From<ItemKindMatchError> for SetSwEffectStateError {
    fn from(error: ItemKindMatchError) -> Self {
        Self::ItemIsNotSwEffect(error)
    }
}
