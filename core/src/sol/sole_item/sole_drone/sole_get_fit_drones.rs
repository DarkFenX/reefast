use crate::{
    err::basic::FitFoundError,
    sol::{FitId, SolarSystem, info::DroneInfo},
};

impl SolarSystem {
    pub fn get_fit_drones(&self, fit_id: &FitId) -> Result<Vec<DroneInfo>, GetFitDronesError> {
        let fit = self.uad.fits.get_fit(fit_id)?;
        let drone_infos = fit
            .drones
            .iter()
            .map(|item_key| self.get_drone_internal(*item_key).unwrap())
            .collect();
        Ok(drone_infos)
    }
}

#[derive(Debug)]
pub enum GetFitDronesError {
    FitNotFound(FitFoundError),
}
impl std::error::Error for GetFitDronesError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::FitNotFound(e) => Some(e),
        }
    }
}
impl std::fmt::Display for GetFitDronesError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::FitNotFound(e) => e.fmt(f),
        }
    }
}
impl From<FitFoundError> for GetFitDronesError {
    fn from(error: FitFoundError) -> Self {
        Self::FitNotFound(error)
    }
}
