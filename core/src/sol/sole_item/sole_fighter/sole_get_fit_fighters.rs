use crate::{
    err::basic::FitFoundError,
    sol::{FitId, SolarSystem, info::FighterInfo},
};

impl SolarSystem {
    pub fn get_fit_fighters(&self, fit_id: &FitId) -> Result<Vec<FighterInfo>, GetFitFightersError> {
        let fit = self.uad.fits.get_fit(fit_id)?;
        let fighter_infos = fit
            .fighters
            .iter()
            .map(|v| self.make_fighter_info(self.uad.items.get_item(v).unwrap().get_fighter().unwrap()))
            .collect();
        Ok(fighter_infos)
    }
}

#[derive(Debug)]
pub enum GetFitFightersError {
    FitNotFound(FitFoundError),
}
impl std::error::Error for GetFitFightersError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::FitNotFound(e) => Some(e),
        }
    }
}
impl std::fmt::Display for GetFitFightersError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::FitNotFound(e) => e.fmt(f),
        }
    }
}
impl From<FitFoundError> for GetFitFightersError {
    fn from(error: FitFoundError) -> Self {
        Self::FitNotFound(error)
    }
}
