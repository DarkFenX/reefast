use crate::{
    err::basic::FitFoundError,
    sol::{
        FitId, FitKey, ItemKey, ItemTypeId, SolarSystem,
        info::CharacterInfo,
        uad::item::{Character, Item},
    },
};

impl SolarSystem {
    pub fn set_fit_character(
        &mut self,
        fit_id: &FitId,
        type_id: ItemTypeId,
        state: bool,
    ) -> Result<CharacterInfo, SetFitCharacterError> {
        let fit_key = self.uad.fits.key_by_id_err(fit_id)?;
        let item_key = self.set_fit_character_internal(fit_key, type_id, state);
        Ok(self.get_character_internal(item_key).unwrap())
    }
    pub(in crate::sol) fn set_fit_character_internal(
        &mut self,
        fit_key: FitKey,
        type_id: ItemTypeId,
        state: bool,
    ) -> ItemKey {
        let fit = self.uad.fits.get(fit_key);
        // Remove old character, if it was set
        if let Some(old_item_key) = fit.character {
            self.remove_character_internal(old_item_key).unwrap();
        }
        // Add new character
        let item_id = self.uad.items.alloc_item_id();
        let character = Character::new(&self.uad.src, item_id, type_id, fit_key, state);
        let item = Item::Character(character);
        let item_key = self.uad.items.add(item);
        let fit = self.uad.fits.get_mut(fit_key);
        fit.character = Some(item_key);
        self.add_item_key_to_svc(item_key);
        item_key
    }
}

#[derive(Debug)]
pub enum SetFitCharacterError {
    FitNotFound(FitFoundError),
}
impl std::error::Error for SetFitCharacterError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::FitNotFound(e) => Some(e),
        }
    }
}
impl std::fmt::Display for SetFitCharacterError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::FitNotFound(e) => e.fmt(f),
        }
    }
}
impl From<FitFoundError> for SetFitCharacterError {
    fn from(error: FitFoundError) -> Self {
        Self::FitNotFound(error)
    }
}
