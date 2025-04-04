use crate::{
    err::basic::{FitFoundError, FitHasItemKindError},
    sol::{FitId, SolarSystem, uad::item::Character},
    util::Named,
};

impl SolarSystem {
    pub fn set_fit_character_state(&mut self, fit_id: &FitId, state: bool) -> Result<(), SetFitCharacterStateError> {
        let fit = self.uad.fits.get_fit(fit_id)?;
        let item_id = match fit.character {
            Some(item_id) => item_id,
            None => {
                return Err(FitHasItemKindError {
                    fit_id: *fit_id,
                    item_kind: Character::get_name(),
                }
                .into());
            }
        };
        let character = self
            .uad
            .items
            .get_item_mut(&item_id)
            .unwrap()
            .get_character_mut()
            .unwrap();
        let old_a_state = character.get_a_state();
        character.set_character_state(state);
        let new_a_state = character.get_a_state();
        self.change_item_id_state_in_svc(&item_id, old_a_state, new_a_state);
        Ok(())
    }
}

#[derive(Debug)]
pub enum SetFitCharacterStateError {
    FitNotFound(FitFoundError),
    FitHasNoCharacter(FitHasItemKindError),
}
impl std::error::Error for SetFitCharacterStateError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::FitNotFound(e) => Some(e),
            Self::FitHasNoCharacter(e) => Some(e),
        }
    }
}
impl std::fmt::Display for SetFitCharacterStateError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::FitNotFound(e) => e.fmt(f),
            Self::FitHasNoCharacter(e) => e.fmt(f),
        }
    }
}
impl From<FitFoundError> for SetFitCharacterStateError {
    fn from(error: FitFoundError) -> Self {
        Self::FitNotFound(error)
    }
}
impl From<FitHasItemKindError> for SetFitCharacterStateError {
    fn from(error: FitHasItemKindError) -> Self {
        Self::FitHasNoCharacter(error)
    }
}
