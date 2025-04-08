use crate::{
    err::basic::{FitFoundError, FitHasItemKindError},
    sol::{FitId, SolarSystem, uad::item::Stance},
    util::Named,
};

impl SolarSystem {
    pub fn remove_fit_stance(&mut self, fit_id: &FitId) -> Result<(), RemoveFitStanceError> {
        let fit = self.uad.fits.get_fit(fit_id)?;
        let item_key = match fit.stance {
            Some(item_key) => item_key,
            None => {
                return Err(FitHasItemKindError {
                    fit_id: *fit_id,
                    item_kind: Stance::get_name(),
                }
                .into());
            }
        };
        Ok(self.remove_stance_internal(item_key).unwrap())
    }
}

#[derive(Debug)]
pub enum RemoveFitStanceError {
    FitNotFound(FitFoundError),
    FitHasNoStance(FitHasItemKindError),
}
impl std::error::Error for RemoveFitStanceError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::FitNotFound(e) => Some(e),
            Self::FitHasNoStance(e) => Some(e),
        }
    }
}
impl std::fmt::Display for RemoveFitStanceError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::FitNotFound(e) => e.fmt(f),
            Self::FitHasNoStance(e) => e.fmt(f),
        }
    }
}
impl From<FitFoundError> for RemoveFitStanceError {
    fn from(error: FitFoundError) -> Self {
        Self::FitNotFound(error)
    }
}
impl From<FitHasItemKindError> for RemoveFitStanceError {
    fn from(error: FitHasItemKindError) -> Self {
        Self::FitHasNoStance(error)
    }
}
