use crate::{
    def::ItemId,
    err::basic::{ItemFoundError, ItemKindMatchError},
    sol::{
        SolarSystem,
        api::{Character, CharacterMut},
    },
};

impl SolarSystem {
    pub fn get_character(&self, item_id: &ItemId) -> Result<Character<'_>, GetCharacterError> {
        let item_key = self.uad.items.key_by_id_err(item_id)?;
        self.uad.items.get(item_key).get_character()?;
        Ok(Character::new(self, item_key))
    }
    pub fn get_character_mut(&mut self, item_id: &ItemId) -> Result<CharacterMut<'_>, GetCharacterError> {
        let item_key = self.uad.items.key_by_id_err(item_id)?;
        self.uad.items.get(item_key).get_character()?;
        Ok(CharacterMut::new(self, item_key))
    }
}

#[derive(thiserror::Error, Debug)]
pub enum GetCharacterError {
    #[error("{0}")]
    ItemNotFound(#[from] ItemFoundError),
    #[error("{0}")]
    ItemIsNotCharacter(#[from] ItemKindMatchError),
}
