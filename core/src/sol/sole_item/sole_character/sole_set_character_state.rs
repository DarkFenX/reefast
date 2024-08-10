use crate::{
    defs::SolItemId,
    sol::{
        err::basic::{ItemFoundError, ItemKindMatchError},
        SolarSystem,
    },
};

impl SolarSystem {
    pub fn set_character_state(&mut self, item_id: &SolItemId, state: bool) -> Result<(), SetCharacterStateError> {
        let character = self.items.get_item_mut(item_id)?.get_character_mut()?;
        let old_state = character.state;
        character.set_bool_state(state);
        let new_state = character.state;
        self.change_item_id_state_in_svcs(item_id, old_state, new_state);
        Ok(())
    }
}

#[derive(Debug)]
pub enum SetCharacterStateError {
    ItemNotFound(ItemFoundError),
    ItemIsNotCharacter(ItemKindMatchError),
}
impl From<ItemFoundError> for SetCharacterStateError {
    fn from(error: ItemFoundError) -> Self {
        Self::ItemNotFound(error)
    }
}
impl From<ItemKindMatchError> for SetCharacterStateError {
    fn from(error: ItemKindMatchError) -> Self {
        Self::ItemIsNotCharacter(error)
    }
}
impl std::error::Error for SetCharacterStateError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::ItemNotFound(e) => Some(e),
            Self::ItemIsNotCharacter(e) => Some(e),
        }
    }
}
impl std::fmt::Display for SetCharacterStateError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::ItemNotFound(e) => e.fmt(f),
            Self::ItemIsNotCharacter(e) => e.fmt(f),
        }
    }
}
